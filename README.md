# SaveFiles
A rust cli used to save directories and files in server.

## Aim

To learn this rust features:
1. cli parsing
2. error management
3. sockets
4. tokio runtime

## Approach

1. create a tcp connection with the server and send the config file which contains username and password key. 
Ther server verifies that and then sends accept data
2. If accepted send the send the text in the file to the remote location specified.
