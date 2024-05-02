desc 'Build '
task :build do
  sh 'cargo rustc -- -C link-args=-Wl,-undefined,dynamic_lookup'
end
