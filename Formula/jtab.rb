class Jtab < Formula
  desc "Print any json data as a table from the command line"
  homepage "https://github.com/wlezzar/jtab"
  bottle :unneeded
  version "0.4.4"

  if OS.mac?
      url "https://github.com/wlezzar/jtab/releases/download/v0.4.4/jtab-v0.4.4-x86_64-apple-darwin.tar.gz"
      sha256 "9ab2e49239ce2aaeeec7928ed4941f5d721e11d80145adbafade95834af311a1"
  elsif OS.linux?
    if Hardware::CPU.intel?
      url "https://github.com/wlezzar/jtab/releases/download/v0.4.4/jtab-v0.4.4-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "f2143d289a82f111a179698adedbecc5a0a5358e59aa3d6874daeec230e45151"
    end
    if Hardware::CPU.arm?
      if Hardware::CPU.is_64_bit?
        url "https://github.com/wlezzar/jtab/releases/download/v0.4.4/jtab-v0.4.4-arm-unknown-linux-gnueabi.tar.gz"
        sha256 "49795cbea0c502b712e8452b4614386157761aa6352fb6fed70234ae3145ae03"
      else
      end
    end
  end

  def install
    bin.install "jtab"
  end
end
