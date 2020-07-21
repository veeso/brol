"""
Developed by Christian Visintin

logrotate-cli is a simple python script which will rotate the log you provide if the size exceeds.
It will create n rotation with syntax LOGFILE.{ROTATION_NUMBER}.log
"""

from io import TextIOWrapper

def struncate(file: TextIOWrapper, amount: int):
    """
    Truncate the first n bytes from the beginning of file

    :param file
    :param amount: amount of bytes to remove from start
    :type file: TextIOWrapper
    :type amount: int
    """
    #Get file size
    file.seek(0, 2)
    file_size = file.tell()
    #Go to the beginning of file
    file_offset = amount
    file.seek(0, 0)
    bytes_to_write = file_size - amount
    bytes_written = 0
    while bytes_written < bytes_to_write:
        #Move to offset + bytes_written
        file.seek(file_offset + bytes_written, 0)
        #Get bytes to rewrite
        block_size = 1024
        if bytes_to_write - bytes_written < block_size:
            block_size = bytes_to_write - bytes_written
        #Read block
        block_data = file.read(block_size)
        #Move to the beginning of file + bytes_written
        file.seek(bytes_written, 0)
        #Write block
        bytes_written += file.write(block_data)
    #Then truncate
    file.flush() #Flush write first
    file.seek(bytes_written)
    file.truncate()
