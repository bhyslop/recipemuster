## RBWHAB — Access Base

**Purpose**
Establish Windows as a keys-only SSH endpoint.

**Preconditions**

* Windows host (admin access)
* Network reachable on TCP/22

**Actions**

1. Install and enable **OpenSSH**:

   ```powershell
   Add-WindowsCapability -Online -Name OpenSSH.Server~~~~0.0.1.0
   Start-Service sshd
   Set-Service -Name sshd -StartupType Automatic
   ```
2. Allow port 22:

   ```powershell
   New-NetFirewallRule -Name sshd -DisplayName "OpenSSH Server" -Enabled True -Direction Inbound -Protocol TCP -Action Allow -LocalPort 22
   ```
3. Configure `sshd_config` (typically `C:\ProgramData\ssh\sshd_config`):

   ```text
   PasswordAuthentication no
   PubkeyAuthentication yes
   PermitEmptyPasswords no
   ChallengeResponseAuthentication no
   UsePAM no
   ```
4. Restart service:

   ```powershell
   Restart-Service sshd
   ```

**Postconditions (invariants)**

* SSH reachable on port 22
* Password login rejected
* Key-based auth required

**Verification**

```bash
ssh user@host
# Expect: publickey prompt; password denied
```

---

## RBWHAR — Access Remote

**Purpose**
Provision client with keys and deterministic host config.

**Preconditions**

* Client machine with SSH

**Actions**

1. Generate key:

   ```bash
   ssh-keygen -t ed25519 -f ~/.ssh/rbwh
   ```
2. Create `~/.ssh/config`:

   ```text
   Host rbwh
     HostName <windows-host-ip>
     User <windows-user>
     IdentityFile ~/.ssh/rbwh
   ```

**Postconditions (invariants)**

* Client has dedicated key
* Host alias `rbwh` resolves correctly

**Verification**

```bash
ssh rbwh
# Expect: connects, but may not yet enter target env (routing not configured)
```

---

## RBWHAX — Access Entrypoints

**Purpose**
Deterministically route SSH keys → environments.

**Preconditions**

* RBWHAB complete
* Public keys available

**Actions**

1. Edit `C:\ProgramData\ssh\administrators_authorized_keys`:

   ```text
   command="C:\cygwin64\bin\bash.exe -l" ssh-ed25519 AAAA... cygwin
   command="wsl.exe -d rbtww-main"       ssh-ed25519 BBBB... wsl
   command="powershell.exe"              ssh-ed25519 CCCC... windows
   ```
2. Set permissions:

   ```powershell
   icacls "C:\ProgramData\ssh\administrators_authorized_keys" /inheritance:r
   icacls "C:\ProgramData\ssh\administrators_authorized_keys" /grant "Administrators:F"
   ```

**Postconditions (invariants)**

* Each key forces a single environment
* No interactive shell selection

**Verification**

```bash
ssh -i key-cygwin rbwh   # lands in Cygwin bash
ssh -i key-wsl rbwh      # lands in WSL (rbtww-main)
ssh -i key-win rbwh      # lands in PowerShell
```

---

## RBWHEW — Environment WSL

**Purpose**
Create canonical Linux environment `rbtww-main`.

**Preconditions**

* Windows admin access

**Actions**

1. Enable WSL:

   ```powershell
   wsl --install
   ```
2. Install Ubuntu:

   ```powershell
   wsl --install -d Ubuntu
   ```
3. Rename/import as `rbtww-main`:

   ```powershell
   wsl --export Ubuntu ubuntu.tar
   wsl --import rbtww-main C:\WSL\rbtww-main ubuntu.tar
   ```
4. Enable systemd:

   ```bash
   sudo nano /etc/wsl.conf
   ```

   ```text
   [boot]
   systemd=true
   ```
5. Restart WSL:

   ```powershell
   wsl --shutdown
   ```

**Postconditions (invariants)**

* Distro `rbtww-main` exists
* systemd active

**Verification**

```bash
wsl -d rbtww-main systemctl is-system-running
# Expect: running or degraded (acceptable)
```

---

## RBWHEU — Environment Users

**Purpose**
Create deterministic multi-user Linux environment.

**Preconditions**

* RBWHEW complete

**Actions**

```bash
sudo adduser alice
sudo adduser bob
sudo usermod -aG sudo alice
sudo usermod -aG sudo bob
```

**Postconditions (invariants)**

* Users `alice`, `bob` exist
* Both have sudo access

**Verification**

```bash
su - alice -c "whoami"
su - bob -c "whoami"
```

---

## RBWHEC — Environment Cygwin

**Purpose**
Install POSIX userland for orchestration testing.

**Preconditions**

* Windows host

**Actions**

1. Install Cygwin to `C:\cygwin64`
2. Install packages:

   * bash
   * openssl
   * curl
3. Verify path:

   ```powershell
   C:\cygwin64\bin\bash.exe -l
   ```

**Postconditions (invariants)**

* Cygwin bash functional
* OpenSSL + curl available

**Verification**

```bash
openssl version
bash --version  # expect 3.x
```

---

## RBWHDD — Docker Desktop

**Purpose**
Provide Windows-hosted Docker daemon.

**Preconditions**

* Windows host

**Actions**

1. Install **Docker Desktop**
2. Enable WSL integration (global)
3. Start Docker Desktop

**Postconditions (invariants)**

* Docker daemon running (Windows VM)

**Verification**

```powershell
docker ps
```

---

## RBWHDW — Docker WSL Native

**Purpose**
Provide Linux-faithful Docker runtime.

**Preconditions**

* RBWHEW complete

**Actions**

```bash
wsl -d rbtww-main
sudo apt update
sudo apt install -y docker.io
sudo systemctl enable --now docker
sudo usermod -aG docker alice
sudo usermod -aG docker bob
```

**Postconditions (invariants)**

* Native `dockerd` running inside WSL
* Users can access Docker

**Verification**

```bash
su - alice -c "docker ps"
```

---

## RBWHDC — Docker Context Discipline

**Purpose**
Ensure deterministic daemon selection.

**Preconditions**

* RBWHDD and RBWHDW complete

**Actions**

1. Inside WSL:

   ```bash
   docker context create wsl-native --docker "host=unix:///var/run/docker.sock"
   docker context use wsl-native
   ```
2. On Windows:

   ```powershell
   docker context use default
   ```

**Postconditions (invariants)**

* WSL shells use native daemon
* Windows/Cygwin use Desktop daemon

**Verification**

```bash
docker context ls
docker info | grep "Server"
```
