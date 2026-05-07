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

# Add proto directory to path for imports
sys.path.insert(0, '/home/admin/RustroverProjects/PlantStation/tools')

try:
    from proto import ps_pb2
except ImportError as e:
    print(f"Error importing protobuf: {e}")
    print("Make sure protobuf is installed: pip install protobuf")
    sys.exit(1)


# Message ID constants (must match MessageId enum in Rust)
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

# StatusType enum mapping (must match StatusType enum in proto)
STATUS_TYPE = {
    'UNKNOWN': 0,
    'I2C': 1,
    'ADC': 2,
}


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
            
            # Parse response
            resp = ps_pb2.GetStatusResp()
            resp.ParseFromString(resp_payload)
            
            print(f"    Status: {resp.status}")
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

