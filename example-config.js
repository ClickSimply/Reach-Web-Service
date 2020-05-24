// this config file controls the ROOT properties for the Reach Web Service.
// The idea here is this is configured once and passed in as argument into RWS app in terminal.

module.exports = {
    // admin_email gets LetsEncrypt notifications, server error notifications, and status updates from the server
    "admin_email": "something@something.com",
    // database, configuration, logs, etc stored here
    "root": "/root/dir/for/rws", 
    // listening ports [http, https]
    "ports":[80, 443], 
    // listening host
    "host": "localhost",
    "default_404": "/html/file/to/load.html",
    "default_500": "/html/file/to/load.html",
    // smtp server details (requires SMTP module)
    "smtp": {
        "host": "smtpserver.com",
        "port": 587,
        "from_name": "Jimmy Jim",
        "from_email": "jimmy@gmail.com",
        "security": "tls",
        "username": "secret_user",
        "password": "secret_pass"
    },
    // access HTTP management API (requires API management module)
    "api_secret": "something crazy and unique here",
    "api_host": "localost",
    "api_port": 9000
}