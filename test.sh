if [ $RUNNER_OS == "ubuntu" ]; then
    tar -zcvf archive/depploy.linux-amd64.tar.gz target/release/depploy
  else
    tar -zcvf archive/depploy.darwin-amd64.tar.gz target/release/depploy
  fi
