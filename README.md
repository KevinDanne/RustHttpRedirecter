# RustHttpRedirector

A simple HTTP request redirector written in rust

## Usage
```
http-redirecter
http-redirecter <ROUTE_FILE_NAME>
```
The routes configuration file must be inside the routes directory.
If no file name is specified "default" is used


## Routes configuration file syntax
```
<REQUEST_PATH> => <REDIRECT_URL>
<REQUEST_PATH_2> => <REDIRECT_URL_2>
```
