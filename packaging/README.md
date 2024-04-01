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
