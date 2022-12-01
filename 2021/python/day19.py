# S3 = {(), (0, 1, 2), (0, 2, 1), (0, 1), (1, 2), (0, 2)}
# Of those: (), (0, 1, 2), (0, 2, 1) are even and (0, 1), (1, 2), (2, 3) are odd
S3_WITH_SIGNS = [(1, (0, 1, 2)), (1, (1, 2, 0)), (1, (2, 0, 1)), (-1, (1, 0, 2)), (-1, (0, 2, 1)), (-1, (2, 1, 0))]
SIGNS = [-1, 1]

def l1_distance(a, b):
    return abs(a[0] - b[0]) + abs(a[1] - b[1]) + abs(a[2] - b[2])

def intersection_size(set1, set2):
    size = 0

    for x in set2:
        if x in set1:
            size += 1

    return size

def generate_rotations():
    # generate "permutation matrices with signs", to quote wikipedia:
    # "Take the set of all 3x3 permutation matrices and assign a + sign or a - 
    # sign to each of the three 1s. There are 6 permutations x 8 sign combinations = 48 matrices in 
    # total giving the full octahedral group. There are exactly 24 matrices with determinant = +1 
    # and these are the rotation matrices of the chiral octahedral group."
    # (https://en.wikipedia.org/wiki/Octahedral_symmetry)
    
    #...except we never actually construct the matrices since they don't
    # really help us (in matrix form) in any case

    for s, (i, j, k) in S3_WITH_SIGNS:
        for s1 in SIGNS:
            for s2 in SIGNS:
                for s3 in SIGNS:
                    # this is the determinant of the "signed permutation matrix"
                    # (the signs of the rows are (s1, s2, s3) and the rows are (e_i, e_j, e_k))
                    if s * s1 * s2 * s3 == -1:
                        continue

                    yield (s1, s2, s3, (i, j, k))

ROTATIONS = list(generate_rotations())

class BeaconSet:
    def __init__(self, beacons):
        self.data = beacons

        # Pre-compute a table of what distances each point in beacons
        # has to any other point in this set.
        self.distances = {}
        for p in self.data:
            self.distances[p] = set(l1_distance(p, q) for q in self.data)

    def __iter__(self):
        for x in self.data:
            yield x

    def rotate_and_translate(self, rot, d):
        s1, s2, s3, (i, j, k) = ROTATIONS[rot]
        d1, d2, d3 = d
        
        new_data = set()
        new_distances = {}
        for x in self.data:
            new_x = (s1 * x[i] - d1, s2 * x[j] - d2, s3 * x[k] - d3)

            new_data.add(new_x)
            new_distances[new_x] = self.distances[x]

        self.data = new_data
        self.distances = new_distances

def find_rotation_and_offset(set1: BeaconSet, set2: BeaconSet):
    # find some rotated version rset2 of set2 and an offset d such that
    # rset2 - d and set1 have an intersection of at least 12 elements.
    for p in set1:
        (p1, p2, p3) = p

        for q in set2:
            # This is a desperate attempt to limit the size of the search space.
            if intersection_size(set1.distances[p], set2.distances[q]) < 12:
                continue

            # p and q *might* be the same point.
            for rot in range(len(ROTATIONS)):
                s1, s2, s3, (i, j, k) = ROTATIONS[rot]

                (d1, d2, d3) = (s1 * q[i] - p1, s2 * q[j] - p2, s3 * q[k] - p3)

                m = 0
                for r in set2:
                    if (s1 * r[i] - d1, s2 * r[j] - d2, s3 * r[k] - d3) in set1:
                        m += 1

                if m >= 12:
                    return rot, (d1, d2, d3)

def solve():
    with open("input/day19") as f:
        lines = f.readlines()
        lines.append('\n')

    scanners = {}
    
    current = None
    current_point_set = set()
    for line in lines:
        line = line.strip()

        if line.startswith("---"):
            split = line.split(' ')
            current = int(split[2])
        elif not line:
            # Line is white space only, we have finished parsing the current scanners section
            scanners[current] = BeaconSet(current_point_set)
            current_point_set = set()
        else:
            x, y, z = [int(c) for c in line.split(',')]
            current_point_set.add((x, y, z))

    n = len(scanners)

    all_beacons = set()
    scanner_offsets = {0:(0, 0, 0)}

    while len(scanner_offsets) < n:
        # Pick any scanner for which we do not know an orientation, and try to find it
        # By comparing it to scanners for which we do know an orientation
        for t in scanners:
            if t in scanner_offsets:
                continue
                
            for s in scanner_offsets:
                res = find_rotation_and_offset(scanners[s], scanners[t])
                if res:
                    # (k1, k2, k3) is the offset of t relative to s
                    rot, offset = res

                    scanners[t].rotate_and_translate(rot, offset)
                    scanner_offsets[t] = offset

                    break


    # Assemble list of all beacons relative to scanner 0
    all_beacons = set()
    for s in scanners:
        all_beacons.update(scanners[s])

    print("Problem 1:", len(all_beacons))

    max_distance = 0

    for s, (p1, p2, p3) in scanner_offsets.items():
        for t, (q1, q2, q3) in scanner_offsets.items():
            if s == t:
                continue

            distance = abs(p1 - q1) + abs(p2 - q2) + abs(p3 - q3)

            if distance > max_distance:
                max_distance = distance

    print("Problem 2:", max_distance)

if __name__ == '__main__':
    solve()
