# BitFabRust

<<<<<<< HEAD
## Current state ->
    - Need to make Uuid seed different for each process
        - This needs to be done because uuid will be same on each peer in each server
    - Need to add router to the server inner it self instead of passing it everytime
=======

## To Do
- [ ] make strea_id unique so that boradcast_to_peers can work
- [x] Move router to sever struct
- [x] Implment the handle_connection in server struct
- [x] Implement the connect_to_peer in server struct

The reason to do them is that we need to access them from the router closures
>>>>>>> b860525644a7ddafad0bd041d85463874ac1abe2
