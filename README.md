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
- Multi machine backup trigger

- Tracing `0.1` seems does not support to set_max_level `yet`.
  - So will do it later.

### Personal Notes
- MountingConfig map to /etc/config can be used for this case.
  - However I don;t know we can use configMap as yaml.
  - https://matthewpalmer.net/kubernetes-app-developer/articles/ultimate-configmap-guide-kubernetes.html
  - https://www.lpalmieri.com/posts/2020-09-27-zero-to-production-4-are-we-observable-yet/#2-observability