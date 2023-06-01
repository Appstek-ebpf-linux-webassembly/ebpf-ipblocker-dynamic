package main

import (
	//"context"
	"log"
	"net"

	pb "grpc-streaming/proto/mypackage"

	"google.golang.org/grpc"
)

type IPInfoServer struct {
	pb.UnimplementedIPInfoServiceServer
}

func (s *IPInfoServer) StreamIPInfo(req *pb.StreamIPRequest, stream pb.IPInfoService_StreamIPInfoServer) error {
	ip := "52.111.244.0"
	port := "8080"

	response := &pb.StreamIPResponse{
		Ip:   ip,
		Port: port,
	}

	for {
		err := stream.Send(response)
		if err != nil {
			return err
		}
	}

	return nil
}

func main() {
	lis, err := net.Listen("tcp", ":50051")
	if err != nil {
		log.Fatalf("Failed to listen: %v", err)
	}

	server := grpc.NewServer()
	pb.RegisterIPInfoServiceServer(server, &IPInfoServer{})

	log.Println("Server started. Listening on port 50051...")
	if err := server.Serve(lis); err != nil {
		log.Fatalf("Failed to serve: %v", err)
	}
}
