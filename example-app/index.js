// This is an example of a javascript application for RWS.
// This is the SERVER side code.


module.exports = {
    name: "app-name", // app-name should be globally unique, placed at host-name/app-name
    description: "some app",
    version: "0.0.1",
    license: "MIT",
    author: "James Kirk",
    author_website: "jameskirk.com",
    dependencies: { // which native modules does this app use and which API version are they on?
        "rws": [1, {}], // [api version, module specific settings/permissions]
        "db": [1, {}],
        "fs": [2, {read: true, write: false}]
    },
    install: (rws) => { // ran when the app is installed

    },
    uninstall: (rws) => { // ran when the app is uninstalled

    },
    host: { // mounted at host-name/app-name/XXX
        "/path": { // mounted at host-name/app-name/path
            auth: (token, req, rws) => {
                // authenticate http/websocket requests to this path
                // if return false, request will be rejected
            },
            http: (token, req, rws) => {
                // handle HTTP requests to host-name/app-name
            },
            ws: (token, client, rws) => {
                // handle websocket requests to host-name/app-name
            },
            dir: "/static" // relative to app root
        },
        "*": { // handle all other requests
            auth: (token, req, rws) => {
                // authenticate http/websocket requests to this path
                // if return false, request will be rejected
            },
            http: (token, req, rws) => {
                // handle HTTP requests to host-name/app-name
            },
            ws: (token, client, rws) => {
                // handle websocket requests to host-name/app-name
            }
        }
    },
    virtual_hosts: { // not mounted anywhere automatically
        named_host: {
            "*": {
                auth: (token, req, rws) => {
                    // authenticate http/websocket requests to this path
                    // if return false, request will be rejected
                },
                http: (token, req, rws) => {
                    // handle HTTP requests to host-name/app-name
                },
                ws: (token, client, rws) => {
                    // handle websocket requests to host-name/app-name
                }
            }
        }
    }
    
}