// Code generated by protoc-gen-go-grpc. DO NOT EDIT.
// versions:
// - protoc-gen-go-grpc v1.3.0
// - protoc             v3.6.1
// source: service.proto

package mypackage

import (
	context "context"
	grpc "google.golang.org/grpc"
	codes "google.golang.org/grpc/codes"
	status "google.golang.org/grpc/status"
)

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
// Requires gRPC-Go v1.32.0 or later.
const _ = grpc.SupportPackageIsVersion7

const (
	IPInfoService_StreamIPInfo_FullMethodName = "/mypackage.IPInfoService/StreamIPInfo"
)

// IPInfoServiceClient is the client API for IPInfoService service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type IPInfoServiceClient interface {
	StreamIPInfo(ctx context.Context, in *StreamIPRequest, opts ...grpc.CallOption) (IPInfoService_StreamIPInfoClient, error)
}

type iPInfoServiceClient struct {
	cc grpc.ClientConnInterface
}

func NewIPInfoServiceClient(cc grpc.ClientConnInterface) IPInfoServiceClient {
	return &iPInfoServiceClient{cc}
}

func (c *iPInfoServiceClient) StreamIPInfo(ctx context.Context, in *StreamIPRequest, opts ...grpc.CallOption) (IPInfoService_StreamIPInfoClient, error) {
	stream, err := c.cc.NewStream(ctx, &IPInfoService_ServiceDesc.Streams[0], IPInfoService_StreamIPInfo_FullMethodName, opts...)
	if err != nil {
		return nil, err
	}
	x := &iPInfoServiceStreamIPInfoClient{stream}
	if err := x.ClientStream.SendMsg(in); err != nil {
		return nil, err
	}
	if err := x.ClientStream.CloseSend(); err != nil {
		return nil, err
	}
	return x, nil
}

type IPInfoService_StreamIPInfoClient interface {
	Recv() (*StreamIPResponse, error)
	grpc.ClientStream
}

type iPInfoServiceStreamIPInfoClient struct {
	grpc.ClientStream
}

func (x *iPInfoServiceStreamIPInfoClient) Recv() (*StreamIPResponse, error) {
	m := new(StreamIPResponse)
	if err := x.ClientStream.RecvMsg(m); err != nil {
		return nil, err
	}
	return m, nil
}

// IPInfoServiceServer is the server API for IPInfoService service.
// All implementations should embed UnimplementedIPInfoServiceServer
// for forward compatibility
type IPInfoServiceServer interface {
	StreamIPInfo(*StreamIPRequest, IPInfoService_StreamIPInfoServer) error
}

// UnimplementedIPInfoServiceServer should be embedded to have forward compatible implementations.
type UnimplementedIPInfoServiceServer struct {
}

func (UnimplementedIPInfoServiceServer) StreamIPInfo(*StreamIPRequest, IPInfoService_StreamIPInfoServer) error {
	return status.Errorf(codes.Unimplemented, "method StreamIPInfo not implemented")
}

// UnsafeIPInfoServiceServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to IPInfoServiceServer will
// result in compilation errors.
type UnsafeIPInfoServiceServer interface {
	mustEmbedUnimplementedIPInfoServiceServer()
}

func RegisterIPInfoServiceServer(s grpc.ServiceRegistrar, srv IPInfoServiceServer) {
	s.RegisterService(&IPInfoService_ServiceDesc, srv)
}

func _IPInfoService_StreamIPInfo_Handler(srv interface{}, stream grpc.ServerStream) error {
	m := new(StreamIPRequest)
	if err := stream.RecvMsg(m); err != nil {
		return err
	}
	return srv.(IPInfoServiceServer).StreamIPInfo(m, &iPInfoServiceStreamIPInfoServer{stream})
}

type IPInfoService_StreamIPInfoServer interface {
	Send(*StreamIPResponse) error
	grpc.ServerStream
}

type iPInfoServiceStreamIPInfoServer struct {
	grpc.ServerStream
}

func (x *iPInfoServiceStreamIPInfoServer) Send(m *StreamIPResponse) error {
	return x.ServerStream.SendMsg(m)
}

// IPInfoService_ServiceDesc is the grpc.ServiceDesc for IPInfoService service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var IPInfoService_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "mypackage.IPInfoService",
	HandlerType: (*IPInfoServiceServer)(nil),
	Methods:     []grpc.MethodDesc{},
	Streams: []grpc.StreamDesc{
		{
			StreamName:    "StreamIPInfo",
			Handler:       _IPInfoService_StreamIPInfo_Handler,
			ServerStreams: true,
		},
	},
	Metadata: "service.proto",
}
