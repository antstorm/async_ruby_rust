require 'bundler'
Bundler.setup(:default)

require 'rutie'

module AsyncRubyRust
  Rutie
    .new(:async_ruby_rust, release: 'debug', lib_path: './target/debug/')
    .init('init_ext', __dir__)
end
