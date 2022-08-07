# hoc
## **Highly** opinionated curl in rust

#### GUIDE:
    
    USAGE:
        hoc [OPTIONS] --url <URL> [METHOD]

    ARGS:
        <METHOD>
                [possible values: get, post, delete, put]

    OPTIONS:
        -u, --url <URL> (The url on which to make request)

        -d, --data <DATA> (Json data)

        -j, --json-header (weather or not to remove Content-type header, -d automatically adds is, this can be used to remove)

        -r, --raw-json (Weather or not to print raw json as output from response, default is pretty)

        -k, --auth-header-key <AUTH_HEADER_KEY> (The header name that should be used for auth, could be Authorization, could be X-Api-Key)

        -v, --auth-header-val <AUTH_HEADER_VAL> (The value of auth header, ie the api key or token or whatever)

        -V, --version
                Print version information

        -h, --help
                Print help information

    

