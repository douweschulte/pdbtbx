from iotbx.pdb import hierarchy as Hierarchy
import timeit
import time
import math
from transformation import matrix_rotate_3d_x, matrix_3d_to_4x4
import numpy as np
import copy


def apply_3d_matrix(position, matrix):
    """
    Applies a 4x4 affine transformation matrix to the given 3D position
    """
    (x, y, z) = position
    new_position = np.dot(matrix,
                          np.matrix([[x], [y], [z], [1]]))
    return (new_position.item(0), new_position.item(1), new_position.item(2))


def transform_protein(hierarchy, transformation, a=None, b=None, c=None):
    """
    Transform the given hierarchy by the given affine transformation matrix
    Returns: nothing, it edits the hierarchy itself
    """
    mod = False
    if a != None:
        mod = True
        assert(b != None)
        assert(c != None)

    for atom in hierarchy.atoms():
        new_position = apply_3d_matrix(atom.xyz, transformation)
        if mod:
            new_position = (
                new_position[0] % a, new_position[1] % b, new_position[2] % c)
        atom.set_xyz(new_position)


def open_pdb(filename):
    Hierarchy.input(file_name=filename)


def transformation(pdb):
    transform_protein(pdb.hierarchy, matrix_3d_to_4x4(
        matrix_rotate_3d_x(90)))


def remove(pdb):
    for atom in pdb.hierarchy.atoms():
        if float(atom.serial) % 2 == 0:
            atom.set_element("DL")

    result = pdb.hierarchy.select(
        pdb.hierarchy.atom_selection_cache().selection("not (element DL)"))


def iteration(pdb):
    average = 0
    for atom in pdb.hierarchy.atoms():
        average += atom.b
    average = average / pdb.hierarchy.atoms_size()


def iteration_build_in(pdb):
    atoms = pdb.hierarchy.atoms()
    average = atoms.extract_b().min_max_mean().mean


def renumber(pdb):
    for chain in pdb.hierarchy.chains():
        chain.id = chain.id[0]

    serial_number = 1
    for atom in pdb.hierarchy.atoms():
        atom.set_serial(serial_number)
        serial_number += 1

    serial_number = 1
    for rg in pdb.hierarchy.residue_groups():
        rg.resseq = serial_number
        serial_number += 1


def clone(pdb):
    copy.deepcopy(pdb)

# No validate


def save(pdb):
    f = open("dump.pdb", "w")
    f.write(pdb.hierarchy.as_pdb_string())
    f.close()


def time_function(fn, subject, name):
    times = []
    start = time.time()
    for i in range(0, 5):
        subject_copy = copy.deepcopy(subject)
        start_inner = time.time()
        result = fn(subject_copy)
        end_inner = time.time()
        times.append(end_inner - start_inner)
    end = time.time()

    runs = math.ceil(5 / (end - start))
    for i in range(0, runs):
        subject_copy = copy.deepcopy(subject)
        start_inner = time.time()
        result = fn(subject_copy)
        end_inner = time.time()
        times.append(end_inner - start_inner)

    avg = sum(times) / (5 + runs)
    stdev = np.std(times)
    print(name + "\t" + str(avg) + "\t" + str(stdev) + "\t" + str(5 + runs))


def time_function_multiple(fn, subjects, global_name):
    for (name, subject) in subjects:
        time_function(fn, subject, global_name + "\t" + name)


names = [
    ("small", "example-pdbs/1ubq.pdb"),
    ("medium", "example-pdbs/1yyf.pdb"),
    ("big", "example-pdbs/pTLS-6484.pdb"),
]

proteins = [
    ("small", Hierarchy.input(file_name="example-pdbs/1ubq.pdb")),
    ("medium", Hierarchy.input(file_name="example-pdbs/1yyf.pdb")),
    ("big", Hierarchy.input(file_name="example-pdbs/pTLS-6484.pdb")),
]

time_function_multiple(open_pdb, names, "open")
time_function_multiple(transformation, proteins, "transformation")
time_function_multiple(remove, proteins, "remove")
time_function_multiple(iteration, proteins, "iteration")
time_function_multiple(iteration_build_in, proteins, "iteration_build_in")
time_function_multiple(renumber, proteins, "renumber")
time_function_multiple(clone, proteins, "clone")
time_function_multiple(save, proteins, "save")
