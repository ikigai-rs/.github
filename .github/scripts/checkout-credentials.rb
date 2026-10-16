#!/usr/bin/env ruby
# checkout-credentials.rb: no workflow in this repo leaves the job token where the
# caller's build can read it (ledger #1725). rust.yml compiles every caller's lock, so
# every build script and proc macro of 50-odd repos runs in its jobs.
#
#   CHECKOUT     every actions/checkout says `persist-credentials: false` (the default
#                writes the token into .git/config for the rest of the job).
#   PERMISSIONS  the workflow declares top-level `permissions`, and it and every job's
#                own grant at most `contents: read`. Undeclared means the caller's
#                default, which can be write-all.
#
# Ruby because its YAML parser ships with it, on the runner and on macOS alike.
# Run: ruby .github/scripts/checkout-credentials.rb [workflow.yml ...]   (exit 0 = clean)
require "yaml"

ALLOWED = { "contents" => "read" }.freeze

def narrow?(perms)
  return false unless perms.is_a?(Hash)
  perms.all? { |k, v| v == "none" || ALLOWED[k] == v }
end

files = ARGV.empty? ? Dir[File.join(__dir__, "..", "workflows", "*.yml")].sort : ARGV
abort "no workflows found: the check would pass on nothing" if files.empty?

findings = []
checkouts = 0
files.each do |path|
  name = File.basename(path)
  doc = YAML.safe_load(File.read(path), aliases: false) || {}
  if !doc.key?("permissions")
    findings << "#{name}: no top-level permissions (the caller's default applies)"
  elsif !narrow?(doc["permissions"])
    findings << "#{name}: top-level permissions #{doc["permissions"].inspect} grant more than contents: read"
  end
  (doc["jobs"] || {}).each do |jid, job|
    if job.key?("permissions") && !narrow?(job["permissions"])
      findings << "#{name} job #{jid}: permissions #{job["permissions"].inspect} grant more than contents: read"
    end
    (job["steps"] || []).each do |step|
      next unless step["uses"].to_s.start_with?("actions/checkout@")
      checkouts += 1
      if (step["with"] || {})["persist-credentials"] != false
        findings << "#{name} job #{jid}: actions/checkout without persist-credentials: false"
      end
    end
  end
end

abort "no actions/checkout found: the check is reading nothing" if checkouts.zero?
if findings.empty?
  puts "#{checkouts} checkouts in #{files.size} workflows: none persists credentials, no grant above contents: read"
else
  findings.each { |f| puts "::error::#{f}" }
  exit 1
end
