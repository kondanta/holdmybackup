# Hold My Backup

It is a backup tool that creates backups and stores them on an object storage. By default it uses `minio` but you can use AWS: S3 as well.


## TODO:
- Reload config without restarting the service.
- Abstract Object Storage implementation
  - Should support atleast AWS: S3 and Minio
- Backup strategy
  - Remove old [ Keep last N backups ]
  - Don't touch
- Folder based backups. (like tar -czvf folder/)
