Small downloader written in Rust for me to learn some cli, http and file io stuff ;)  

To use this downloader, you have to create a ".env" file with following content:  
OUTPUT_DIR=[WHERE_ALL_FILES_ARE_DOWNLOADED]  
QUEUE_FILE=[WHAT_TO_DOWNLOAD]  

The queue file will be downloaded line by line and every existing file (matched by name) will be skipped.  
The queue files content will not be erased upon completion.  

One queue line has to be formatted like following:  
[FILE_NAME];[URL]
