# warrant-canary
A simple warrant canary/ signed policy builder in Rust. Designed to facilitate quickly making a HTML canary notice.

This command line application will build and output a HTML page containing a signed warrant canary. The included policy and announcements are for example only and may not be appropriate for everyone.

## Usage
The application takes four command line arguments: The URL to your website, the expiration of the canary in days, the name or email for your GPG signing key, and the path to output the HTML file. Note that the output directory must already exist. 

Example: $ ./warrant-canary example.com 30 Edward_Snowden /var/www/canary/index.html

The canary_elements directory must also exist in the same directory as the executable and contain the anouncements.txt and policy.txt.

You will be prompted for your GPG signing key password. GPG signing is handled with commands passed to the host machine so you will need GPG installed and signing key available. This application has only been tested on the Bash shell but should work on similar shell environments. This application requires internet access to pull headline articles from newsnationnow.com to be used for freshness.

# NOTICE This application is for demonstration purposes only
This project is just a proof of concept and for me to work on my Rust skills. I am not an attorney and the use of such a canary may not be wise or legal. 
The effectiveness of warrant canaries is a topic of much debate and the practical use of such has not been tested in court.
A much better solution is to not keep or have access to any information that you would not want to hand over to authorities. 


![image](https://github.com/patrickramp/warrant-canary/assets/142554235/1d5d431c-ae32-44b1-b6d9-dbedd47a5257)
