#!/usr/bin/env ruby

=begin

Compiles the plugin for all supported architectures and assembles a plugin bundle

=end

require 'fileutils'

PLUGIN_NAME = 'rust_plugin'

class Target
    attr_accessor :triple, :platform, :bits, :file_name_format
    def initialize(triple, platform, bits, file_name_format)
        @triple = triple
        @platform = platform
        @bits = bits
        @file_name_format = file_name_format
    end
end

# Creates a directory if it does not exist
def mkdir_p(path)
    if !Dir::exist? path
        Dir::mkdir path
    end
end

def mkdirs(plugin_name, targets)
    plugin_dir = "#{PLUGIN_NAME}.bundle"
    mkdir_p plugin_dir
    Dir::chdir plugin_dir
    targets.each do |target|
        mkdir_p target.bits
    end
    Dir::chdir '..'
end

TARGETS = [
    Target::new('x86_64-apple-darwin', 'mac', '64', 'lib%s.dylib'),
    Target::new('i686-apple-darwin', 'mac', '32', 'lib%s.dylib'),
    Target::new('x86_64-unknown-linux-gnu', 'lin', '64', 'lib%s.so'),
    Target::new('i686-unknown-linux-gnu', 'lin', '32', 'lib%s.so'),
]

# Set up directory structure
mkdirs(PLUGIN_NAME, TARGETS)

# Compile
TARGETS.each do |target|
    puts "Building #{target.triple}"
    result = system "cargo build --release --target #{target.triple}"
    if !result
        puts "Build failed"
        exit
    end
    # Move binary into plugin directory
    binary_name = target.file_name_format % PLUGIN_NAME
    FileUtils.mv "target/#{target.triple}/release/#{binary_name}",
     "#{PLUGIN_NAME}.bundle/#{target.bits}/#{target.platform}.xpl"
end
