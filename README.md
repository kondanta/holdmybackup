# Hold My Backup

It is a backup tool that creates backups and stores them on an object storage. By default it uses `minio` but you can use AWS: S3 as well.

## Purpose

It is actually a pet project for understanding core concepts of `cloud native` applications. I don't have any
practicle use case for this app, but one can use this for manage their backups I guess. PR's are welcomed.

## Showcase

### Here's how it looks:

- Create Backup:
![Creating Backup](https://i.imgur.com/3rQVgec.gif)

- Changing log level
![Changing Log Level](https://i.imgur.com/PztUv9C.gif)

- Listing available backups
![Listing Existing Backups](https://i.imgur.com/ajBxGC8.gif)

## TODO:
- [x] Reload config without restarting the service.
- [x] Abstract Object Storage implementation
   - Should support atleast AWS: S3 and Minio
     - [x] minio
     - [] AWS: S3
- [x] Dynamically change log type
  - Change log level to one of the available log levels anytime through HTTP PUT request
  - Available log levels: trace, debug, info, war, error
- [ ] Backup strategy
  - Remove old [ Keep last N backups ]
  - Don't touch
- [x] Folder based backups. (like tar -czvf folder/)
- [ ] Export tracing logs using OTeL format.
  - Jeager
- [ ] Create instance metrics and export them as well.
  - Prometheus?
  - Implement RED
