import subprocess
import glob
import os
import pathlib
import sys
from PIL import Image

OUTPUT_DIR = "output"


def by_file_id(file: str) -> int:
    file_name = pathlib.Path(file).name.split(".")[0].split("_")[0]
    return int(file_name) if "final" not in file_name else sys.maxsize


def main() -> None:
    for f in [f for f in os.listdir(OUTPUT_DIR) if "resized" not in f]:
        num = f.split(".")[0]
        subprocess.run(
            [
                "convert",
                f"{OUTPUT_DIR}/{num}.png",
                "-filter",
                "point",
                "-resize",
                "600%",
                f"{OUTPUT_DIR}/{num}_resized.png",
            ],
            check=True,
        )

    frames = glob.glob(f"{OUTPUT_DIR}/*_resized.png")
    frames.sort(key=by_file_id)
    frames = [Image.open(os.getcwd() + "/" + image) for image in frames]
    frame_one = frames[0]
    frame_one.save(
        "animated.gif",
        format="GIF",
        append_images=frames,
        save_all=True,
        duration=16,
    )


if __name__ == "__main__":
    main()
