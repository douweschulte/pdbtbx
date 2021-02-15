#!/usr/bin/python
# Code written by dschulte for PyTom and reused here, original source: https://github.com/FridoF/PyTom/blob/master/basic/combine_transformations.py

import numpy as np


def combine_trans_projection(tx: float, ty: float, rot: float, mag: float, x: float, y: float, z: float, phi: float, the: float, psi: float, tiltangle: float, dim: float, binning: int, particle_dim: int = 200):
    """
    Combines all transformations of the raw o aligned and the position and rotations of a particle into one single matrix
    to efficiently and with only one round of interpolation be able to compare a template with raw data.
    @param tiltangle: The tiltangle
    @param dim: The dimensions of the raw data (assumes a square)
    @param binning: The binning factor of the coordinates of the particle position
    @param: particle_dim: The dimension of the template (assumes a cube)
    @return: (align transformations, particle position in raw data, particle position in aligned data, matrix for template)
    @author: Douwe Schulte and Gijs van der Schot
    """
    from numpy import cos, sin, pi

    # Calculates the inverse transformation matrix of the projection alignment transformations
    alpha = -rot * pi/180
    c = cos(alpha)
    s = sin(alpha)

    rotate = np.matrix([[c, s, 0],  [-s, c, 0],   [0, 0, 1]])
    magnify = np.matrix([[mag, 0, 0], [0, mag, 0], [0, 0, 1]])
    translate = np.matrix([[1, 0, tx],  [0, 1, ty],  [0, 0, 1]])

    align_transformations = np.linalg.inv(rotate * magnify * translate)

    # Map the 3D position to a 2D position on the projection of the tiltangle
    x = x * binning
    y = y * binning
    z = z * binning

    aligned_y = y  # assume the rotation axis is around y
    aligned_x = (cos(tiltangle * pi / 180) * (x - dim / 2) -
                 sin(tiltangle * pi / 180) * (z - dim / 2)) + dim / 2

    # Use the projection alignment transformations to map this 2D position to a 2D position on the raw projections
    aligned_pos = np.matrix([[aligned_x - dim/2], [aligned_y - dim/2], [1]])
    raw_pos = align_transformations * aligned_pos

    # Calculate the rotation matrix for the template, a combination of the particle rotation and the tilt angle
    template_3d_rotation = generate_rotation_matrix(0, tiltangle, 0) * generate_rotation_matrix(
        phi, the, psi) * matrix_rotate_3d_z(rot) * matrix_magnify_3d(mag)

    # Merge this matrix with the projection transformations
    merged_matrix = template_3d_rotation

    return (align_transformations, (raw_pos.item(0, 0) + dim/2, raw_pos.item(1, 0) + dim/2), (aligned_x, aligned_y), merged_matrix)


def matrix_rotate_3d_x(deg: float) -> np.matrix:
    """Creates a 3d 3x3 rotation matrix for a deg turn on the x axis"""
    from numpy import cos, sin, pi
    rad_x = -deg * pi/180
    c_x = cos(rad_x)
    s_x = sin(rad_x)
    return np.matrix([[1, 0, 0], [0, c_x, -s_x], [0, s_x, c_x]])


def matrix_rotate_3d_y(deg: float) -> np.matrix:
    """Creates a 3d 3x3 rotation matrix for a deg turn on the y axis"""
    from numpy import cos, sin, pi
    rad_y = -deg * pi/180
    c_y = cos(rad_y)
    s_y = sin(rad_y)
    return np.matrix([[c_y, 0, s_y], [0, 1, 0], [-s_y, 0, c_y]])


def matrix_rotate_3d_z(deg: float) -> np.matrix:
    """Creates a 3d 3x3 rotation matrix for a deg turn on the z axis"""
    from numpy import cos, sin, pi
    rad_z = -deg * pi/180
    c_z = cos(rad_z)
    s_z = sin(rad_z)
    return np.matrix([[c_z, -s_z, 0], [s_z, c_z, 0], [0, 0, 1]])


def matrix_translate_3d(tx: float, ty: float, tz: float) -> np.matrix:
    """Creates a 3d 4x4 affine transformation matrix for a 3d translation"""
    return np.matrix([[1, 0, 0, tx], [0, 1, 0, ty], [0, 0, 1, tz], [0, 0, 0, 1]])


def matrix_magnify_3d(f: float) -> np.matrix:
    """Creates a 3d 3x3 rotation matrix for a magnification in every axis"""
    return np.matrix([[f, 0, 0], [0, f, 0], [0, 0, f]])


def matrix_2d_to_3d(matrix: np.matrix) -> np.matrix:
    """Calculates the 3d affine transformation matrix from the given 2d affine transformation matrix"""
    return np.matrix([
        [matrix.item(0, 0), matrix.item(0, 1), 0, matrix.item(0, 2)],
        [matrix.item(1, 0), matrix.item(1, 1), 0, matrix.item(1, 2)],
        [0,                 0,                 1, 0],
        [matrix.item(2, 0), matrix.item(2, 1), 0, matrix.item(2, 2)]])


def matrix_3d_to_4x4(matrix: np.matrix) -> np.matrix:
    """Calculates the 3d 4x4 affine transformation matrix from the given 3d 3x3 affine rotation matrix"""
    return np.matrix([
        [matrix.item(0, 0), matrix.item(0, 1), matrix.item(0, 2), 0],
        [matrix.item(1, 0), matrix.item(1, 1), matrix.item(1, 2), 0],
        [matrix.item(2, 0), matrix.item(2, 1), matrix.item(2, 2), 0],
        [0,                 0,                 0,                 1]])


def matrix_apply_to_3d_4x4(vol, matrix: np.matrix):
    from scipy import mgrid

    # Calculate the new coordinates of every point
    grid = mgrid[0.:vol.shape[0], 0.:vol.shape[1], 0.:vol.shape[2]]
    temp = grid.reshape((3, grid.size / 3))
    # Add the fourth dimension (just 1s but needed for the computations)
    newrow = np.ones(grid.size / 3)
    temp = np.vstack([temp, newrow])
    # Use the matrix to calculate the new positions of every point
    temp = np.dot(matrix, temp)
    # Delete the fourth dimension
    temp = np.delete(temp, 3, axis=0)
    temp = np.array(temp)
    grid = np.reshape(temp, (3, vol.shape[0], vol.shape[1], vol.shape[2]))

    from scipy.ndimage.interpolation import map_coordinates
    d = map_coordinates(vol, grid, order=3)

    return d


def matrix_apply_to_3d_3x3(vol, matrix: np.matrix):
    """Applies a given 3d 3x3 rotation matrix to the given volume, rotating around the center"""
    from scipy import mgrid

    cx = vol.shape[0]/2
    cy = vol.shape[1]/2
    cz = vol.shape[2]/2

    # Calculate the new coordinates of every point
    grid = mgrid[-cx:vol.shape[0]-cx, -cy:vol.shape[1]-cy, -cz:vol.shape[2]-cz]
    temp = grid.reshape((3, grid.size / 3))
    # Add the fourth dimension (just 1s but needed for the computations)
    # Use the matrix to calculate the new positions of every point
    temp = np.dot(matrix, temp)
    # Delete the fourth dimension
    temp = np.array(temp)
    grid = np.reshape(temp, (3, vol.shape[0], vol.shape[1], vol.shape[2]))

    grid[0] += cx
    grid[1] += cy
    grid[2] += cz

    from scipy.ndimage.interpolation import map_coordinates
    d = map_coordinates(vol, grid, order=3)

    return d


def matrix_apply_to_2d(data, matrix: np.matrix):
    """Applies a given 2d 2x2 rotation matrix to the given array, rotating around the center"""
    from scipy import mgrid

    cx = data.shape[0] / 2
    cy = data.shape[1] / 2

    # Calculate the new coordinates of every point
    grid = mgrid[-cx:data.shape[0]-cx, -cy:data.shape[1]-cy]
    temp = grid.reshape((2, grid.size / 2))
    # Add the fourth dimension (just 1s but needed for the computations)
    newrow = np.ones(grid.size / 2)
    temp = np.vstack([temp, newrow])
    # Use the matrix to calculate the new positions of every point
    temp = np.dot(matrix, temp)
    # Delete the fourth dimension
    temp = np.delete(temp, 2, axis=0)
    temp = np.array(temp)
    grid = np.reshape(temp, (2, data.shape[0], data.shape[1]))

    grid[0] += cx
    grid[1] += cy

    from scipy.ndimage.interpolation import map_coordinates
    d = map_coordinates(data, grid, order=3)

    return d


def generate_rotation_matrix(phi: float, the: float, psi: float) -> np.matrix:
    """Creates a 3d 3x3 rotation matrix with the given rotations in ZXZ notation."""
    # Transfer the angle to Euclidean
    phi = -float(phi) * np.pi / 180.0
    the = -float(the) * np.pi / 180.0
    psi = -float(psi) * np.pi / 180.0
    sin_alpha = np.sin(phi)
    cos_alpha = np.cos(phi)
    sin_beta = np.sin(the)
    cos_beta = np.cos(the)
    sin_gamma = np.sin(psi)
    cos_gamma = np.cos(psi)

    # Calculate inverse rotation matrix
    Inv_R = np.zeros((3, 3), dtype='float32')

    Inv_R[0, 0] = cos_alpha * cos_gamma - cos_beta * sin_alpha \
        * sin_gamma
    Inv_R[0, 1] = -cos_alpha * sin_gamma - cos_beta * sin_alpha \
        * cos_gamma
    Inv_R[0, 2] = sin_beta * sin_alpha

    Inv_R[1, 0] = sin_alpha * cos_gamma + cos_beta * cos_alpha \
        * sin_gamma
    Inv_R[1, 1] = -sin_alpha * sin_gamma + cos_beta * cos_alpha \
        * cos_gamma
    Inv_R[1, 2] = -sin_beta * cos_alpha

    Inv_R[2, 0] = sin_beta * sin_gamma
    Inv_R[2, 1] = sin_beta * cos_gamma
    Inv_R[2, 2] = cos_beta
    #Inv_R[3, 3] = 1

    return np.matrix(Inv_R)
