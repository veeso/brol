#!/usr/bin/python3

from io import BytesIO
from sys import argv, exit
from pydub import AudioSegment
from pydub.silence import split_on_silence
import speech_recognition as sr
from typing import List
from yaspin import yaspin

if __name__ == "__main__":
    if len(argv) < 4:
        print("Usage: <input_audio_file> <min_length> <treshold>")
        exit(255)
    input_file = argv[1]
    min_length = int(argv[2])
    treshold = int(argv[3])
    # Open audio
    audio = AudioSegment.from_file(input_file)
    # Split audio
    with yaspin(text="Splitting audio into chunks...") as spinner:
        chunks: List[AudioSegment] = split_on_silence(audio, min_length, treshold)
    print("Found %d chunks" % len(chunks))
    speech_recognizer = sr.Recognizer()
    for i, chunk in enumerate(chunks):
        print()
        with yaspin(text="Getting speech for chunk %d" % i) as spinner:
            audio_data = BytesIO()
            audio_data = chunk.export(audio_data, "wav")
            sr_audio = sr.AudioFile(audio_data)
            with sr_audio as source:
                try:
                    audio_file = speech_recognizer.record(source)
                    result: str = speech_recognizer.recognize_google(
                        audio_file, language="it_IT"
                    )
                    spinner.ok("✔️")
                    print("%d: %s" % (i, result))
                except Exception as e:
                    spinner.fail("❌")
                    print("Failed to get speech for %d: %s" % (i, e))
