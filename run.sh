#!/usr/bin/env /bin/bash
Green='\033[1;92m'    # Green
Reset='\033[0m'       # Text Reset

function check_folders_exist {
  if [ ! -d "instances" ] || [ ! -d "instances/ISC" ] || [ ! -d "instances/SC" ] || [ ! -d "instances/UC" ] || [ ! -d "instances/WC" ]; then
      echo "Required folders do not exist. Please download instance following the instructions in the README.md file."
      exit 1
  fi
}

function check_relative_folder_exists {
  instance_name=$1
  if [ ! -d "instances/$instance_name" ]; then
    echo "Folder 'instances/$instance_name' does not exist."
    exit 1
  fi
}

function setup {
  cargo build -r
  check_folders_exist
}

function generate_csv_filename {
  echo "results_$(date +%Y%m%d_%H%M%S).csv"
}

function test_all {
  check_folders_exist
  csv_file=$(generate_csv_filename)

  for folder in instances/*; do
    if [ -d "$folder" ]; then
      for file in "$folder"/*; do
        if [ -f "$file" ]; then
          for seed in {1..10}; do
            echo "Running cargo for $file with seed $seed"
            cargo run -r -- -f "$file" -s "$seed" -r "$csv_file" -l "error"
          done
        fi
      done
    fi
  done

  echo "metrics saved in $csv_file"
}

function test_specific_instance {
  instance_kind=$1
  check_relative_folder_exists "$instance_kind"
  csv_file="$instance_kind.$(generate_csv_filename)"

  echo "metrics saved in $csv_file"
  for file in instances/$instance_kind/*; do
    if [ -f "$file" ]; then
      for seed in {1..10}; do
        echo "Running cargo for $file with seed $seed"
        cargo run -r -- -f "$file" -s "$seed" -r "$csv_file" -l "error"
      done
    fi
  done

  echo "metrics saved in $csv_file"
}


while [[ $# -gt 0 ]]
do
  key="$1"

  case "$key" in
    setup)
      setup
      exit
    ;;
    all)
      echo "Starting testing all instances..."
      test_all
      exit
    ;;
    relative)
      if [ -z "$2" ]; then
        echo "Instance name is required for relative testing."
        exit 1
      fi
      echo "Starting testing instances... $2"
      test_specific_instance "$2"
      exit
      ;;
  esac

  shift
done

echo -en """
Commands:
  - ${Green}setup${Reset}: installs the necessary dependencies
  - ${Green}all${Reset}: performs tests on all instances kind
  - ${Green}relative <instances-kind>${Reset}: performs tests on specific instances kind
"""