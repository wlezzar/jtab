class Jtab < Formula
  desc "Print any json data as a table from the command line"
  homepage "https://github.com/wlezzar/jtab"
  version "0.5.1"

  if OS.mac?
    if Hardware::CPU.intel?
      url "https://github.com/wlezzar/jtab/releases/download/v0.5.1/jtab-0.5.1-x86_64-apple-darwin.zip"
      sha256 "9ba8294485847824e08f654723f841c3cda4b843625de670379fd824e3036fc6"
    end
    if Hardware::CPU.arm?
      url "https://github.com/wlezzar/jtab/releases/download/v0.5.1/jtab-0.5.1-aarch64-apple-darwin.zip"
      sha256 "7dec6cf02f39b5e7f09d9945c7a0531d06d0caa6cedd96f58bcdc1269ba168a2"
    end
  elsif OS.linux?
    if Hardware::CPU.intel?
      url "https://github.com/wlezzar/jtab/releases/download/v0.5.1/jtab-0.5.1-x86_64-unknown-linux-gnu.zip"
      sha256 "cb2efad8454704ad3f3a3490698fe2d6043d9f0cac87f963e28094e21e3e4889"
    end
    if Hardware::CPU.arm?
      if Hardware::CPU.is_64_bit?
        url "https://github.com/wlezzar/jtab/releases/download/v0.5.1/jtab-0.5.1-aarch64-unknown-linux-gnu.zip"
        sha256 "0edd19a008f3b8e3464aef37a68d685b8eb9334902e037d275fc08e2511ce074"
      else
      end
    end
  end

  def install
    bin.install Dir["bin/jtab"]
  end
end
