#!/usr/bin/python3

from sys import argv, exit
from pydub import AudioSegment
from pydub.silence import split_on_silence
from yaspin import yaspin

if __name__ == "__main__":
    if len(argv) < 5:
        print("Usage: <input_audio_file> <out_dir> <min_length> <treshold>")
        exit(255)
    input_file = argv[1]
    out_dir = argv[2]
    min_length = int(argv[3])
    treshold = int(argv[4])
    # Open audio
    audio = AudioSegment.from_file(input_file)
    # Split audio
    with yaspin(text = "Splitting audio into chunks...") as spinner:
        chunks: list = split_on_silence(audio, min_length, treshold)
    print("Found %d chunks" % len(chunks))
    for i, chunk in enumerate(chunks):
        with yaspin(text = "Exporting chunk %d" % i) as spinner:
            out_file = "%s/chunk%d.wav" % (out_dir, i)
            chunk.export(out_file, format="wav")
            spinner.ok("OK")
