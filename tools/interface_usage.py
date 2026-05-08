#!/usr/bin/env python3
"""
PlantStation API Interface Usage Script

Demonstrates how to use the PlantStation API via protobuf-based UDP protocol.
This script builds requests, serializes them with protobuf, sends them to the
server, and deserializes responses.
"""

import socket
import sys
import argparse
from typing import Optional, Tuple
import time

sys.path.insert(0, '/home/admin/RustroverProjects/PlantStation/tools')

try:
    from proto import ps_pb2
except ImportError as e:
    print(f"Error importing protobuf: {e}")
    print("Make sure protobuf is installed: pip install protobuf")
    sys.exit(1)


MESSAGE_ID = {
    'GetStatusReq': 1,
    'GetStatusResp': 2,
    'GetAdcValueReq': 3,
    'GetAdcValueResp': 4,
    'GetHygrometerStatusReq': 5,
    'GetHygrometerStatusResp': 6,
    'GetTemperatureReq': 7,
    'GetTemperatureResp': 8,
}

STATUS_TYPE = {
    'UNKNOWN': 0,
    'I2C': 1,
    'ADC': 2,
}

mux_names = {
    0: "AIN0/GND",
    1: "AIN1/GND",
    2: "AIN2/GND",
    3: "AIN3/GND",
    4: "AIN0/AIN1",
    5: "AIN0/AIN3",
    6: "AIN1/AIN3",
    7: "AIN2/AIN3"
}

pga_names = {
    0: "±6.144V",
    1: "±4.096V",
    2: "±2.048V",
    3: "±1.024V",
    4: "±0.512V",
    5: "±0.256V",
    6: "±0.256V",
    7: "±0.256V"
}

dr_names = {
    0: "8 SPS",
    1: "16 SPS",
    2: "32 SPS",
    3: "64 SPS",
    4: "128 SPS",
    5: "250 SPS",
    6: "475 SPS",
    7: "860 SPS"
}

def process_status(status: str, status_type: str) -> str:
    if status_type == "I2C":
        if " | " in status:
            parts = status.split(" | ")
            dev_path = parts[0]
            flags = parts[1:]
            
            formatted = f"Device: {dev_path}\n    Functions:\n"
            for flag in flags:
                formatted += f"      • {flag}\n"
            return formatted.rstrip()
        else:
            return f"I2C Status: {status}"
    
    elif status_type == "ADC":
        if status.startswith("0x") or status.startswith("0X"):
            try:
                hex_str = status
                config_value = int(hex_str, 16)
                
                # Decode ADS1115 Config register bits (16-bit)
                # Bit 15: OS (Operational Status)
                # Bits 14-12: MUX (Input Multiplexer Configuration)
                # Bits 11-9: PGA (Programmable Gain Amplifier)
                # Bit 8: MODE (Device Operating Mode)
                # Bits 7-5: DR (Data Rate)
                # Bit 4: COMP_MODE (Comparator Mode)
                # Bit 3: COMP_POL (Comparator Polarity)
                # Bit 2: COMP_LAT (Latching Comparator)
                # Bits 1-0: COMP_QUE (Comparator Queue)
                
                os_bit = (config_value >> 15) & 0x1
                mux_bits = (config_value >> 12) & 0x7
                pga_bits = (config_value >> 9) & 0x7
                mode_bit = (config_value >> 8) & 0x1
                dr_bits = (config_value >> 5) & 0x7
                comp_mode = (config_value >> 4) & 0x1
                comp_pol = (config_value >> 3) & 0x1
                comp_lat = (config_value >> 2) & 0x1
                comp_que = (config_value >> 0) & 0x3
                

                formatted = f"ADC Config Register: {hex_str}\n"
                formatted += f"    Operational Status (OS): {'Converting' if os_bit else 'Ready'}\n"
                formatted += f"    Input Multiplexer (MUX): {mux_names.get(mux_bits, 'Unknown')}\n"
                formatted += f"    Programmable Gain (PGA): {pga_names.get(pga_bits, 'Unknown')}\n"
                formatted += f"    Operating Mode: {'Single-shot' if mode_bit else 'Continuous'}\n"
                formatted += f"    Data Rate (DR): {dr_names.get(dr_bits, 'Unknown')}\n"
                formatted += f"    Comparator Mode: {'Window' if comp_mode else 'Traditional'}\n"
                formatted += f"    Comparator Polarity: {'Active Low' if comp_pol else 'Active High'}\n"
                formatted += f"    Comparator Latching: {'Yes' if comp_lat else 'No'}\n"
                formatted += f"    Comparator Queue: {comp_que} {'assertion' if comp_que < 3 else 'disabled'}"
                
                return formatted
            except (ValueError, IndexError):
                return f"ADC Error: Invalid hex format '{status}'"
        else:
            return f"ADC Error: {status}"
    
    else:
        return status


class PlantStationClient:
    """Client for communicating with PlantStation server via UDP."""

    def __init__(self, host: str = '127.0.0.1', port: int = 8080, timeout: float = 2.0):
        """
        Initialize the PlantStation client.
        
        Args:
            host: Server host address
            port: Server port
            timeout: Socket timeout in seconds
        """
        self.host = host
        self.port = port
        self.timeout = timeout
        self.sock = None

    def connect(self):
        """Open UDP socket connection."""
        self.sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        self.sock.settimeout(self.timeout)
        print(f"[*] Connected to {self.host}:{self.port}")

    def disconnect(self):
        """Close UDP socket."""
        if self.sock:
            self.sock.close()
            print("[*] Disconnected")

    def _send_request(self, msg_id: int, payload: bytes) -> Tuple[int, bytes]:
        """
        Send a request and receive a response.
        
        Args:
            msg_id: Message ID as first byte
            payload: Protobuf-encoded payload
            
        Returns:
            Tuple of (response_msg_id, response_payload)
        """
        # Frame: [MessageID] + [Protobuf Payload]
        request = bytes([msg_id]) + payload
        
        print(f"    Sending {len(request)} bytes: {request.hex()}")
        self.sock.sendto(request, (self.host, self.port))
        
        # Receive response
        data, addr = self.sock.recvfrom(1024)
        print(f"    Received {len(data)} bytes from {addr}: {data.hex()}")
        
        if len(data) < 1:
            raise RuntimeError("Empty response received")
        
        response_msg_id = data[0]
        response_payload = data[1:]
        
        return response_msg_id, response_payload

    def get_status(self, status_type: str) -> Optional[ps_pb2.GetStatusResp]:
        """
        Get hardware status (I2C or ADC).
        
        Args:
            status_type: 'I2C' or 'ADC'
            
        Returns:
            GetStatusResp message or None on error
        """
        print(f"[*] Getting {status_type} status...")
        
        if status_type not in STATUS_TYPE:
            print(f"    ERROR: Invalid status type '{status_type}'. Choose 'I2C' or 'ADC'")
            return None
        
        # Build request
        req = ps_pb2.GetStatusReq()
        req.status_type = STATUS_TYPE[status_type]
        
        try:
            resp_msg_id, resp_payload = self._send_request(
                MESSAGE_ID['GetStatusReq'],
                req.SerializeToString()
            )
            
            resp = ps_pb2.GetStatusResp()
            resp.ParseFromString(resp_payload)
            print(f"    Status: {process_status(resp.status, status_type)}")
            return resp
            
        except socket.timeout:
            print("    ERROR: Request timeout")
            return None
        except Exception as e:
            print(f"    ERROR: {e}")
            return None

    def get_adc_value(self, channel: int = 0, converted: bool = True) -> Optional[ps_pb2.GetAdcValueResp]:
        """
        Get ADC value from specified channel.
        
        Args:
            channel: ADC channel (0-7)
            converted: If True, return voltage; if False, return raw value
            
        Returns:
            GetAdcValueResp message or None on error
        """
        print(f"[*] Getting ADC value from channel {channel} (converted={converted})...")
        
        # Build request
        req = ps_pb2.GetAdcValueReq()
        req.converted = converted
        req.channel = channel
        
        try:
            resp_msg_id, resp_payload = self._send_request(
                MESSAGE_ID['GetAdcValueReq'],
                req.SerializeToString()
            )
            
            # Parse response
            resp = ps_pb2.GetAdcValueResp()
            resp.ParseFromString(resp_payload)
            
            print(f"    Value: {resp.value}")
            return resp
            
        except socket.timeout:
            print("    ERROR: Request timeout")
            return None
        except Exception as e:
            print(f"    ERROR: {e}")
            return None

    def get_humidity(self, channel: int = 0) -> Optional[ps_pb2.GetHygrometerStatusResp]:
        """
        Get humidity reading from specified channel.
        
        Args:
            channel: Hygrometer channel (0-7)
            
        Returns:
            GetHygrometerStatusResp message or None on error
        """
        print(f"[*] Getting humidity from channel {channel}...")
        
        # Build request
        req = ps_pb2.GetHygrometerStatusReq()
        req.channel = channel
        
        try:
            resp_msg_id, resp_payload = self._send_request(
                MESSAGE_ID['GetHygrometerStatusReq'],
                req.SerializeToString()
            )
            
            # Parse response
            resp = ps_pb2.GetHygrometerStatusResp()
            resp.ParseFromString(resp_payload)
            
            print(f"    Humidity: {resp.humidity}%")
            return resp
            
        except socket.timeout:
            print("    ERROR: Request timeout")
            return None
        except Exception as e:
            print(f"    ERROR: {e}")
            return None

    def get_temperature(self) -> Optional[ps_pb2.GetTemperatureResp]:
        """
        Get temperature reading.
        
        Returns:
            GetTemperatureResp message or None on error
        """
        print("[*] Getting temperature...")
        
        # Build request
        req = ps_pb2.GetTemperatureReq()
        req.dummy = 0
        
        try:
            resp_msg_id, resp_payload = self._send_request(
                MESSAGE_ID['GetTemperatureReq'],
                req.SerializeToString()
            )
            
            # Parse response
            resp = ps_pb2.GetTemperatureResp()
            resp.ParseFromString(resp_payload)
            
            print(f"    Temperature: {resp.temperature}°C")
            return resp
            
        except socket.timeout:
            print("    ERROR: Request timeout")
            return None
        except Exception as e:
            print(f"    ERROR: {e}")
            return None


def main():
    """Main function with CLI interface."""
    parser = argparse.ArgumentParser(
        description='PlantStation API Client - Protocol Buffer Interface Usage Example'
    )
    parser.add_argument('--host', default='127.0.0.1', help='Server host (default: 127.0.0.1)')
    parser.add_argument('--port', type=int, default=8080, help='Server port (default: 8080)')
    parser.add_argument('--timeout', type=float, default=2.0, help='Request timeout in seconds (default: 2.0)')
    parser.add_argument('--status', choices=['I2C', 'ADC'], help='Get status (I2C or ADC)')
    parser.add_argument('--temperature', action='store_true', help='Get temperature')
    parser.add_argument('--humidity', type=int, metavar='CHANNEL', help='Get humidity from channel')
    parser.add_argument('--adc', type=int, metavar='CHANNEL', help='Get ADC value from channel')
    parser.add_argument('--converted', action='store_true', help='For ADC: get converted value (voltage)')
    parser.add_argument('--demo', action='store_true', help='Run demo with all commands')
    
    args = parser.parse_args()
    
    # Create client
    client = PlantStationClient(host=args.host, port=args.port, timeout=args.timeout)
    
    try:
        client.connect()
        
        # Run commands
        if args.demo:
            print("\n=== PlantStation API Demo ===\n")
            print("[1] Testing I2C Status")
            client.get_status('I2C')
            time.sleep(0.5)
            
            print("\n[2] Testing ADC Status")
            client.get_status('ADC')
            time.sleep(0.5)
            
            print("\n[3] Testing ADC Value (Raw)")
            client.get_adc_value(channel=0, converted=False)
            time.sleep(0.5)
            
            print("\n[4] Testing ADC Value (Converted/Voltage)")
            client.get_adc_value(channel=0, converted=True)
            time.sleep(0.5)
            
            print("\n[5] Testing Humidity")
            client.get_humidity(channel=0)
            time.sleep(0.5)
            
            print("\n[6] Testing Temperature")
            client.get_temperature()
            
        else:
            if args.status:
                client.get_status(args.status)
            
            if args.temperature:
                client.get_temperature()
            
            if args.humidity is not None:
                client.get_humidity(channel=args.humidity)
            
            if args.adc is not None:
                client.get_adc_value(channel=args.adc, converted=args.converted)
            
            if not any([args.status, args.temperature, args.humidity is not None, args.adc is not None]):
                print("No command specified. Use --demo for a demo or use specific options.")
                parser.print_help()
        
    except Exception as e:
        print(f"ERROR: {e}")
        sys.exit(1)
    finally:
        client.disconnect()


if __name__ == '__main__':
    main()

