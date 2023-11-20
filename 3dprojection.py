import numpy as np
import matplotlib.pyplot as plt
from matplotlib.animation import FuncAnimation
# from mpl_toolkits.mplot3d import Axes3D

# Define the rotation matrix
def rotation_matrix(theta, axis):
    axis = axis.lower()
    if axis == 'x':
        return np.array([[1, 0, 0, 0],
                         [0, np.cos(theta), -np.sin(theta), 0],
                         [0, np.sin(theta), np.cos(theta), 0],
                         [0, 0, 0, 1]])
    elif axis == 'y':
        return np.array([[np.cos(theta), 0, np.sin(theta), 0],
                         [0, 1, 0, 0],
                         [-np.sin(theta), 0, np.cos(theta), 0],
                         [0, 0, 0, 1]])
    elif axis == 'z':
        return np.array([[np.cos(theta), -np.sin(theta), 0, 0],
                         [np.sin(theta), np.cos(theta), 0, 0],
                         [0, 0, 1, 0],
                         [0, 0, 0, 1]])
    else:
        raise ValueError("Axis must be 'x', 'y', or 'z'")



def translation_matrix(x, y, z):
    return np.array([[1, 0, 0, x],
                     [0, 1, 0, y],
                     [0, 0, 1, z],
                     [0, 0, 0, 1]])


def perspective_matrix(fov, aspect, n, f):
    t = np.tan(np.radians(fov)/2) * abs(n)
    b = -t
    r = t * aspect
    l = -r
    return np.array([[2*n/(r-l), 0, (r+l)/(r-l), 0],
                     [0, 2*n/(t-b), (t+b)/(t-b), 0],
                     [0, 0, -(f+n)/(f-n), -2*f*n/(f-n)],
                     [0, 0, -1, 0]])



def orthographic_matrix(l, r, b, t, n, f):
    return np.array([[2/(r-l), 0, 0, -(r+l)/(r-l)],
                     [0, 2/(t-b), 0, -(t+b)/(t-b)],
                     [0, 0, -2/(f-n), -(f+n)/(f-n)],
                     [0, 0, 0, 1]])







# Define cube vertices
cube_vertices = np.array([[-1, -1, 1, 1],
                          [1, -1, 1, 1],
                          [1, 1, 1, 1],
                          [-1, 1, 1, 1],
                          [-1, -1, -1, 1],
                          [1, -1, -1, 1],
                          [1, 1, -1, 1],
                          [-1, 1, -1, 1]])

# Define cube edges
cube_edges = np.array([[0, 1],
                       [1, 2],
                       [2, 3],
                       [3, 0],
                       [4, 5],
                       [5, 6],
                       [6, 7],
                       [7, 4],
                       [0, 4],
                       [1, 5],
                       [2, 6],
                       [3, 7]])

translation = translation_matrix(0, 0, 5)
perspective = perspective_matrix(40, 1, 1, 100)
orthographic = orthographic_matrix(-1, 1, -1, 1, 1, 100)


# Initialize figure and 3d axis
fig = plt.figure()
ax = fig.add_subplot(111, projection='3d')
# ax = fig.add_subplot(111)

# Set the limits of the axes
ax.set_xlim([-3, 3])
ax.set_ylim([-3, 3])
ax.set_zlim([-3, 3])
# Initialize cube plot
# cube_lines = [ax.plot([], [], [], c='teal')[0] for _ in range(len(cube_edges))]
cube_lines = [ax.plot([], [], c='r')[0] for _ in range(len(cube_edges))]

# Animation update function
def update(frame):
    # pipeline
    theta = np.radians(frame)
    rotationy = rotation_matrix(theta, 'y')
    rotationx = rotation_matrix(theta, 'x')
    rotation = np.dot(rotationx, rotationy)
    modelview = translation @ rotation
    new_vertices = np.dot(cube_vertices, modelview.T)
    # new_vertices = np.dot(new_vertices, perspective.T)
    new_vertices /= new_vertices[:, 3].reshape(-1, 1)  # Divide by w to normalize
    for edge, line in zip(cube_edges, cube_lines):
        line.set_data(new_vertices[edge, 0:2].T) # x and y
        line.set_3d_properties(new_vertices[edge, 2].T) # z
        # line.set_3d_properties(np.zeros(2)) # z

# Create animation
ani = FuncAnimation(fig, update, frames=np.arange(0, 360, 2), interval=50)

# To save the animation, use the following line
# ani.save('cube_rotation.gif', writer='imagemagick')
plt.title('Cube rotation')
plt.show()
