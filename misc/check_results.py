import os
import sys

if len(sys.argv) != 4:
	print("ERROR: Not provided 3 required arguments - " + str(sys.argv))	
	exit(1)

folder_name = sys.argv[1]
files_required = sys.argv[2]
directories_required = sys.argv[3]

file_count = sum(len(files) for _, _, files in os.walk(folder_name))
if str(file_count) != files_required:
	print("Current files: " + str(file_count) + ", but required is: " + str(files_required))
	print("This commit probably introduced regression, please recheck it.")
	exit(1)

directory_count = sum(len(dire) for _, dire, files in os.walk(folder_name))
if str(directory_count) != directories_required:
	print("Current directories: " + str(directory_count) + ", but required is: " + str(directories_required))
	print("This commit probably introduced regression, please recheck it.")
	exit(1)

