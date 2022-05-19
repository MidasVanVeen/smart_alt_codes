# Smart alt codes

Program that reads input from a device file, usually located in /dev/input/by-path or /dev/input/by-id on linux devices, and recognised sequences specified in a csv file. The program will then replace the typed sequence with whatever value is specified in the second column of the csv file.

The patterns.csv file is an example of such a csv file.

The program will start recording a sequence when the right alt key is pressed, this key can be modified in main.rs.