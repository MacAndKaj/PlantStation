"""PlantStation Protobuf Messages

This package contains auto-generated protobuf message definitions for the PlantStation API.
Generated from proto/ps.proto using protoc.

Usage:
    from proto import ps_pb2
    
    # Create a request
    req = ps_pb2.GetStatusReq()
    req.status_type = 1  # I2C
    
    # Serialize
    payload = req.SerializeToString()
    
    # Deserialize
    resp = ps_pb2.GetStatusResp()
    resp.ParseFromString(payload)
"""

from . import ps_pb2

__all__ = ['ps_pb2']

