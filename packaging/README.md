# Packaging

- [archlinux](archlinux)

## Archlinux

### custom-ddns

```bash
cp -r archlinux/custom-ddns $HOME/.cache/custom-ddns-git
cd $HOME/.cache/custom-ddns-git

makepkg # -i
```

Once installed:
- edit `/etc/custom-ddns/prod.yaml`
- systemctl start custom-ddns
- systemctl status custom-ddns

**Systemd unit service uses a dynamic user and can't start with a port below 1024; it is recommanded to use an nginx or an alternative in front of this service**

### gandictl

```bash
cp -r archlinux/gandictl $HOME/.cache/gandictl-git
cd $HOME/.cache/gandictl-git

makepkg # -i
```

2 packages will be created:
- gandictl-git: gandictl controls the gandi.net management console
- gandictl-ddns-git: Gandi Dynamic DNS Updater

To use Gandi Dynamic DNS Updater:
- edit `/etc/conf.d/gandictl-ddns`
- systemctl enable gandictl-ddns.timer
