class Jtab < Formula
  desc "Print any json data as a table from the command line"
  homepage "https://github.com/wlezzar/jtab"
  bottle :unneeded
  version "0.3.0"

  if OS.mac?
      url "https://github.com/wlezzar/jtab/releases/download/v0.3.0/jtab-v0.3.0-x86_64-apple-darwin.tar.gz"
      sha256 "3eb5568caafab2c217d8da939ad35b4f9f550639e0defb37e28133fb9363eaa6"
  elsif OS.linux?
    if Hardware::CPU.intel?
      url "https://github.com/wlezzar/jtab/releases/download/v0.3.0/jtab-v0.3.0-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "1eb9f4d7911795e8c6a45a3b329b86af4fb3a5db82b21ed40a17fc76281ec1ed"
    end
    if Hardware::CPU.arm?
      if Hardware::CPU.is_64_bit?
        url "https://github.com/wlezzar/jtab/releases/download/v0.3.0/jtab-v0.3.0-arm-unknown-linux-gnueabi.tar.gz"
        sha256 "4a447a716ebefad57ccb83f1e8c6f16ffe0be7a01a38a48649453738be77e08e"
      else
      end
    end
  end

  def install
    bin.install "jtab"
  end
end
