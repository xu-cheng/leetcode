require "pathname"
require "shellwords"

task default: :build

desc "build"
task :build do
  sh "cargo", "+nightly", "build"
end

desc "test <question>"
task :test, [:question] do |_task, args|
  sh "cargo", "+nightly", "test", *[args[:question]].compact
end

desc "bench <question>"
task :bench, [:question] do |_task, args|
  sh "cargo", "+nightly", "bench", *[args[:question]].compact
end

desc "generate answer template"
task :new do
  question = nil
  title = nil
  loop do
    puts "Input question number and title (e.g. \"1. question title\"):"
    input = $stdin.readline.strip
    question = input[/^(\d+)\./, 1].to_i
    title = input[/\. (.*)$/, 1]
    break if question > 0 && !title.empty?
  end

  question_str = "_#{question.to_s.rjust(3, "0")}"
  question_file = Pathname.new("./src/#{question_str}.rs")
  lib_file = Pathname.new("./src/lib.rs")
  raise "#{question_file} already existing." if question_file.file?
  raise "#{lib_file} not found." unless lib_file.file?

  lib_src = lib_file.read.lines
  insert_line_no = nil
  lib_src.each_with_index do |line, line_no|
    q_no = line[/^pub mod _(\d+); \/\/ .*$/, 1]
    next if q_no.nil?

    q_no = q_no.to_i
    insert_line_no = line_no if q_no < question
  end
  lib_src.insert(insert_line_no + 1, "pub mod #{question_str}; // #{title}\n")
  lib_file.write(lib_src.join)
  question_file.write <<~EOS
    pub struct Solution;

    impl Solution {

    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_solution() {

        }
    }
  EOS
  sh "#{ENV["EDITOR"]} #{question_file.to_s.shellescape}" if ENV["EDITOR"] && !ENV["EDITOR"].empty?
end
