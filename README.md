# ebpf-ipblocker
Root Folder contain 3 folder/directory
1)grpc-stream
  a)Server
  b)client

2)Policy
  Folder Contains output.json file which is generated by grpc stream

3)xdp-drop
  ebpf code to block ip ,generated by grpc and stored in output.json

Run the application IPBlocker(xdp-drop)

step 1  Enter the directory xdp-drop  

step 2  pass the command ip a to get the name of your   
        network device name 
        output simmilar to below
        1:lo   2:enp0s31f6   3)wlp113s0

step 3  Select 2 or 3 depend upon your choice for LAN and  
        wifi NC

Run the command (console 1)

xdp-drop$ RUST_LOG=info cargo xtask run -- --iface wlp113s0(debug mode)


sudo -E RUST_LOG=debug target/release/xdp-drop --iface wlp113s0(release mode)
------------------------------------------------------------

Run the application grpc stream to fatch list of ip aadress dynamically into IPBlocker
In a new console (console 2)
switch to directory grpc-stream/server 

 Run the command 
 grpc-stream/server$ go run server.go
 **********
 To add new ip to block change the code 

 func (s *IPInfoServer) StreamIPInfo(req *pb.StreamIPRequest, stream pb.IPInfoService_StreamIPInfoServer) error {
	ip := "35.163.22.86"
	port := "8080"
 }
 Take any ip which is frequently appere in terminal log 
 **********

in new console
switch to directory grpc-stream/client
run the command (console 3)
grpc-stream/client$ go run client.go
----------------------------------------------
once client.json is complete its execution 
check the Policy/output.json file which contain the ip which is send by server .

Now again check the xdp-drop console 1 to varify that ip is blocked and Action indicate 1 : Block : 2 : UnBlock





 
