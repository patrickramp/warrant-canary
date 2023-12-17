# warrant-canary
A simple warrant canary/ signed policy builder in Rust. To facilitate making an HTML notice.  

This command line application will build a HTML page containing a signed warant canary. The included polocy and anouncemnts are for example only and may not be appropreate for everyone.

The application takes four command line arugments: the URL to your website, the name or email for your GPG key, the experation of the canary in days and the path to the output html. Note that the output directory must already exist.

Example: $ ./warrant-canary example.com Edward_Snowden 30 /var/www/canary/index.html

You will be prompted for your GPG signing key password. GPG signing is handeled with commands passed to the host machine so you will need GPG installed and signing key
available. This applicaiton has only been tested on the Bash shell but should work on similar shell enviroments. This application requires internet access to pull headline articals from newsnationnow.com to be used for freshness.

## This application is for demonstration purposes only
This project is just a proof of consept and for me to work on my Rust skills. I am not an attorny and the use of such a cannary may not be wise or legal. 
The effectiveness of warrant canaries is a topic of much debate and the practical use of such has not been tested in court.
A much better solution is to not keep or have access to any infomration that you would not want to hand over to authorites. 
![image](https://github.com/patrickramp/warrant-canary/assets/142554235/1d5d431c-ae32-44b1-b6d9-dbedd47a5257)
