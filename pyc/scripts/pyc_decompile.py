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


class Counter(object):
    def __init__(self, initval=0):
        from multiprocessing import Value, Lock
        self.val = Value('i', initval)
        self.lock = Lock()

    def increment(self):
        with self.lock:
            self.val.value += 1

    def value(self):
        with self.lock:
            return self.val.value


def init(l, fp, ff, fo):
    global lock
    global files_processed
    global files_failed
    global files_okay
    files_processed = fp
    files_failed = ff
    files_okay = fo
    lock = l


def decompile_file_async(filename, out_base):
    import sys
    try:
        lock.acquire()
        print("\r", end="")
        sys.stdout.write("\033[K")
        print("okay: %d failed: %d %s" %
              (files_okay.value(), files_failed.value(), filename), end="\r")
        sys.stdout.flush()
    finally:
        lock.release()

    try:
        result = decompile_file(filename, out_base)
        # lock.acquire()
        if result:
            files_okay.increment()
        else:
            files_failed.increment()
    except:
        files_failed.increment()
    finally:
        files_processed.increment()
        # lock.release()

    import gc
    gc.collect()

    try:
        lock.acquire()
        print("\r", end="")
        sys.stdout.write("\033[K")
        print("okay: %d failed: %d %s" %
              (files_okay.value(), files_failed.value(), filename), end='\r')
        sys.stdout.flush()
    finally:
        lock.release()


def decompile_file_async_unpack(args):
    return decompile_file_async(*args)


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

    from multiprocessing import Pool, Lock

    # Create pool for parallel execution
    # The lock here is used to synchronize directory create calls
    lock = Lock()
    fp = Counter(0)
    ff = Counter(0)
    fo = Counter(0)
    import multiprocessing
    pool = Pool(int(multiprocessing.cpu_count() / 2),
                initializer=init, initargs=(lock, fp, ff, fo,))

    init(lock, fp, ff, fo)

    files = []
    for filename in glob.glob(args.glob_pattern, recursive=True):
        # Skipping these as it chokes in Uncompyle6, with incredibly high memory usage
        # They seem to decompuile fine with pycdc so if they are interesting, use that
        if 'row_x_x_random_names_data' in filename:
            continue
        if 'space_area_rgb_data' in filename:
            continue
        if 'QA_way_point' in filename:
            continue
        if 'waypoint_data' in filename:
            continue
        # if 'ukeys_with_table_data.py' in filename:
        #     continue
        files.append((filename, args.out_base))

    pool.map_async(decompile_file_async_unpack, files).get(999999)

    sys.stdout.write("\033[K")
    print(
        f"total: {files_processed.value()} okay: {files_okay.value()} failed: {files_failed.value()}")
    sys.stdout.flush()

# decompile_file.__jit__()
