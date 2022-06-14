import glob
import os


def add_import():
    import sys
    import os
    dir_path = os.path.dirname(os.path.realpath(__file__))

    sys.path.insert(0, os.path.join(dir_path, "modules"))


add_import()

import uncompyle6  # noqa


def _get_outstream(outfile: str):
    dir = os.path.dirname(outfile)
    failed_file = outfile + "_failed"
    if os.path.exists(failed_file):
        os.remove(failed_file)
    try:
        os.makedirs(dir)
    except OSError:
        pass
    return open(outfile, mode="w", encoding="utf-8")


def decompile_file(filename: str, out_base="."):
    try:
        if filename.endswith(".pyc"):
            current_outfile = os.path.join(out_base, filename[0:-1])
        else:
            current_outfile = os.path.join(out_base, filename) + "_dis"

        outstream = _get_outstream(current_outfile)
        uncompyle6.decompile_file(filename, outstream)
        return True
    except:
        return False


def decompile_multiple_files(filenames, out_base):
    import threading
    threads = []
    for filename in filenames:
        t1 = threading.Thread(target=decompile_file, args=(filename, out_base))
        threads.append(t1)

    for x in threads:
        x.start()

    # Wait for all of them to finish
    for x in threads:
        x.join()


if __name__ == "__main__":
    import sys
    import argparse
    parser = argparse.ArgumentParser(description='')
    parser.add_argument('glob_pattern', type=str,  help='')
    parser.add_argument('--out-base', dest='out_base', default='.')

    args = parser.parse_args()
    failed = 0
    okay = 0
    last_filename = ''
    for filename in glob.glob(args.glob_pattern, recursive=True):
        last_filename = filename
        print("\r", end="")
        sys.stdout.write("\033[K")
        print(f"okay: {okay} failed: {failed} {last_filename}", end="\r")
        sys.stdout.flush()
        try:
            if decompile_file(filename, args.out_base):
                okay += 1
            else:
                failed += 1
        except:
            failed += 1

    print("\r", end="")
    sys.stdout.write("\033[K")
    print(f"okay: {okay} failed: {failed} {last_filename}", end="\r")
    sys.stdout.flush()

# decompile_file.__jit__()
