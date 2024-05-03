require 'bundler'
Bundler.setup(:default)

require 'rutie'

class AsyncRubyRust
  Rutie
    .new(:async_ruby_rust, release: 'debug', lib_path: './target/debug/')
    .init('init_ext', __dir__)
end

async = AsyncRubyRust.init
Thread.new { async.run_callback_loop }

async.sleep(5_000);

