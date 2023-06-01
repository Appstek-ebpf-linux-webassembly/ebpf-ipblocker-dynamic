package main

import (
	"context"
	"encoding/json"
	"io/ioutil"
	"log"

	pb "grpc-streaming/proto/mypackage"

	"google.golang.org/grpc"
)

func main() {
	conn, err := grpc.Dial(":50051", grpc.WithInsecure())
	if err != nil {
		log.Fatalf("Failed to connect: %v", err)
	}
	defer conn.Close()

	client := pb.NewIPInfoServiceClient(conn)
	stream, err := client.StreamIPInfo(context.Background(), &pb.StreamIPRequest{})
	if err != nil {
		log.Fatalf("Error while calling StreamIPInfo: %v", err)
	}

	var data []map[string]string

	for i := 1; i <= 2; i++ {
		response, err := stream.Recv()
		if err != nil {
			log.Fatalf("Error while receiving response: %v", err)
		}

		ip := response.GetIp()
		port := response.GetPort()

		info := map[string]string{
			"ip":   ip,
			"port": port,
		}
		data = append(data, info)

		log.Printf("Received IP: %s, Port: %s\n", ip, port)
	}

	jsonData, err := json.MarshalIndent(data, "", "  ")
	if err != nil {
		log.Fatalf("Failed to marshal data to JSON: %v", err)
	}

	err = ioutil.WriteFile("../.././Policy/output.json", jsonData, 0644)
	if err != nil {
		log.Fatalf("Failed to write output file: %v", err)
	}

	log.Println("JSON file created successfully.")
}
