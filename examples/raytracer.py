import numpy as np
import matplotlib.pyplot as plt

def sqrt(x):
    last_guess= x/2.0
    while True:
        guess= (last_guess + x/last_guess)/2
        checkable = guess - last_guess
        checkable_abs = checkable
        if checkable < 0:
            checkable_abs = 0 - checkable
        if checkable_abs < .000001: # example threshold
            return guess
        last_guess= guess

def normalize(x,y,z):
    #print(vector)
    #print(np.linalg.norm(vector))
    xnorm = x / sqrt(x**2 + y**2 + z**2)
    ynorm = y / sqrt(x**2 + y**2 + z**2)
    znorm = z / sqrt(x**2 + y**2 + z**2)
    
    return np.array([xnorm, ynorm, znorm])

def dot_product(x1,y1,z1, x2,y2,z2):
    result = x1 * x2 + y1 * y2 + z1 * z2
    return result


def sphere_intersect(centerx, centery, centerz, radius, origx, origy, origz, dirx, diry, dirz):
    b = 2 * dot_product(dirx, diry, dirz, origx - centerx, origy - centery, origz - centerz)
    
    normalised = sqrt((origx - centerx)**2 + (origy - centery)**2 + (origz - centerz)**2)
    
    c = normalised ** 2 - radius ** 2
    delta = b ** 2 - 4 * c
    if delta > 0:
        t1 = (-b + sqrt(delta)) / 2
        t2 = (-b - sqrt(delta)) / 2
        if t1 > 0 and t2 > 0:
            return min(t1, t2)
    return None

def do_spheres_intersect(centerx, centery, centerz, radius, origx, origy, origz, dirx, diry, dirz):
    b = 2 * dot_product(dirx, diry, dirz, origx - centerx, origy - centery, origz - centerz)
    
    normalised = sqrt((origx - centerx)**2 + (origy - centery)**2 + (origz - centerz)**2)
    
    c = normalised ** 2 - radius ** 2
    delta = b ** 2 - 4 * c
    if delta > 0:
        t1 = (-b + sqrt(delta)) / 2
        t2 = (-b - sqrt(delta)) / 2
        if t1 > 0 and t2 > 0:
            return True
    else:
        return False


obj1_center_x = -0.2
obj1_center_y = 0
obj1_center_z = -1
obj1_rad = 0.7
obj1_ambient = np.array([0.1, 0, 0])
obj1_diffuse = np.array([0.7, 0, 0])

obj2_center_x = 0.1
obj2_center_y = -0.3
obj2_center_z = 0
obj2_rad = 0.1
obj2_ambient = np.array([0.1, 0, 0.1])
obj2_diffuse = np.array([0.7, 0, 0.7])

obj3_center_x = -0.3
obj3_center_y = 0
obj3_center_z = 0
obj3_rad = 0.15
obj3_ambient = np.array([0, 0.1, 0])
obj3_diffuse = np.array([0, 0.6, 0])

obj4_center_x = 0
obj4_center_y = -9000
obj4_center_z = 0
obj4_rad = 9000 - 0.7
obj4_ambient = np.array([0.1,0.1,0.1])
obj4_diffuse = np.array([0.6, 0.6, 0.6])

specular = np.array([1,1,1])
shinines = 100
obj_reflection = 0.5

def does_intersect(ray_origin, ray_direction):
    inter0 = do_spheres_intersect(obj1_center_x, obj1_center_y, obj1_center_z, obj1_rad, ray_origin[0], ray_origin[1], ray_origin[2], ray_direction[0], ray_direction[1], ray_direction[2])
    inter1 = do_spheres_intersect(obj2_center_x, obj2_center_y, obj2_center_z, obj2_rad, ray_origin[0], ray_origin[1], ray_origin[2], ray_direction[0], ray_direction[1], ray_direction[2])
    inter2 = do_spheres_intersect(obj3_center_x, obj3_center_y, obj3_center_z, obj3_rad, ray_origin[0], ray_origin[1], ray_origin[2], ray_direction[0], ray_direction[1], ray_direction[2])
    inter3 = do_spheres_intersect(obj4_center_x, obj4_center_y, obj4_center_z, obj4_rad, ray_origin[0], ray_origin[1], ray_origin[2], ray_direction[0], ray_direction[1], ray_direction[2])
    if inter0 or inter1 or inter2 or inter3:
        return True
    else:
        return False

def nearest_intersected_object(ray_origin, ray_direction):
    distance0 = sphere_intersect(obj1_center_x, obj1_center_y, obj1_center_z, obj1_rad, ray_origin[0], ray_origin[1], ray_origin[2], ray_direction[0], ray_direction[1], ray_direction[2])
    distance1 = sphere_intersect(obj2_center_x, obj2_center_y, obj2_center_z, obj2_rad, ray_origin[0], ray_origin[1], ray_origin[2], ray_direction[0], ray_direction[1], ray_direction[2])
    distance2 = sphere_intersect(obj3_center_x, obj3_center_y, obj3_center_z, obj3_rad, ray_origin[0], ray_origin[1], ray_origin[2], ray_direction[0], ray_direction[1], ray_direction[2])
    distance3 = sphere_intersect(obj4_center_x, obj4_center_y, obj4_center_z, obj4_rad, ray_origin[0], ray_origin[1], ray_origin[2], ray_direction[0], ray_direction[1], ray_direction[2])
    min_distance = 1000000000000.0
    
    inter0 = do_spheres_intersect(obj1_center_x, obj1_center_y, obj1_center_z, obj1_rad, ray_origin[0], ray_origin[1], ray_origin[2], ray_direction[0], ray_direction[1], ray_direction[2])
    inter1 = do_spheres_intersect(obj2_center_x, obj2_center_y, obj2_center_z, obj2_rad, ray_origin[0], ray_origin[1], ray_origin[2], ray_direction[0], ray_direction[1], ray_direction[2])
    inter2 = do_spheres_intersect(obj3_center_x, obj3_center_y, obj3_center_z, obj3_rad, ray_origin[0], ray_origin[1], ray_origin[2], ray_direction[0], ray_direction[1], ray_direction[2])
    inter3 = do_spheres_intersect(obj4_center_x, obj4_center_y, obj4_center_z, obj4_rad, ray_origin[0], ray_origin[1], ray_origin[2], ray_direction[0], ray_direction[1], ray_direction[2])
   
    nearest_object_index = 4
    
    if inter0:
        if distance0 < min_distance:
            min_distance = distance0
            nearest_object_index = 0
    if inter1:
        if distance1 < min_distance:
            min_distance = distance1
            nearest_object_index = 1
    if inter2:
        if distance2 < min_distance:
            min_distance = distance2
            nearest_object_index = 2
    if inter3:
        if distance3 < min_distance:
            min_distance = distance3
            nearest_object_index = 3   

    return nearest_object_index, min_distance

width = 100
height = 100

max_depth = 3

camera = np.array([0, 0, 1])
ratio = float(width) / height
screen = (-1, 1 / ratio, 1, -1 / ratio) # left, top, right, bottom

light = { 'position': np.array([5, 5, 5]), 'ambient': np.array([1, 1, 1]), 'diffuse': np.array([1, 1, 1]), 'specular': np.array([1, 1, 1]) }

image = np.zeros((height, width, 3))
for i, y in enumerate(np.linspace(screen[1], screen[3], height)):
    for j, x in enumerate(np.linspace(screen[0], screen[2], width)):
        # screen is on origin
        pixel = np.array([x, y, 0])
        origin = camera
        tonormx= pixel[0] - origin[0]
        tonormy = pixel[1] - origin[1]
        tonormz = pixel[2] - origin[2]
        
        directionx = tonormx / sqrt(tonormx**2 + tonormy**2 + tonormz**2)
        directiony = tonormy / sqrt(tonormx**2 + tonormy**2 + tonormz**2)
        directionz = tonormz / sqrt(tonormx**2 + tonormy**2 + tonormz**2)
        

        color = np.zeros((3))
        reflection = 1

        for k in range(max_depth):
            # check for intersections
            if does_intersect(origin, np.array([directionx, directiony, directionz])):
                nearest_object, min_distance = nearest_intersected_object(origin, np.array([directionx, directiony, directionz]))
                if nearest_object is None:
                   break

                intersection = origin + min_distance * np.array([directionx, directiony, directionz])
            
                if nearest_object == 0: 
                    to_norm_x = intersection[0] - obj1_center_x
                    to_norm_y = intersection[1] - obj1_center_y
                    to_norm_z = intersection[2] - obj1_center_z
                
                if nearest_object == 1: 
                    to_norm_x = intersection[0] - obj2_center_x
                    to_norm_y = intersection[1] - obj2_center_y
                    to_norm_z = intersection[2] - obj2_center_z
                
                if nearest_object == 2: 
                    to_norm_x = intersection[0] - obj3_center_x
                    to_norm_y = intersection[1] - obj3_center_y
                    to_norm_z = intersection[2] - obj3_center_z
                
                if nearest_object == 3: 
                    to_norm_x = intersection[0] - obj4_center_x
                    to_norm_y = intersection[1] - obj4_center_y
                    to_norm_z = intersection[2] - obj4_center_z
            
                normx = to_norm_x / sqrt(to_norm_x**2 + to_norm_y**2 + to_norm_z**2)
                normy = to_norm_y / sqrt(to_norm_x**2 + to_norm_y**2 + to_norm_z**2)
                normz = to_norm_z / sqrt(to_norm_x**2 + to_norm_y**2 + to_norm_z**2)
            
                normal_to_surface = np.array([normx, normy, normz])
                shifted_point = intersection + 1e-5 * normal_to_surface
                intersection_to_light = normalize(light['position'][0] - shifted_point[0], light['position'][1] - shifted_point[1],light['position'][2] - shifted_point[2])

                _, min_distance = nearest_intersected_object(shifted_point, intersection_to_light)
                intersection_to_light_distance = np.linalg.norm(light['position'] - intersection)
                is_shadowed = min_distance < intersection_to_light_distance

                if is_shadowed:
                    break

                illumination = np.zeros((3))

            # ambiant
                if nearest_object == 0:
                    illumination += obj1_ambient * light['ambient']
                if nearest_object == 1:
                    illumination += obj2_ambient * light['ambient']
                if nearest_object == 2:
                    illumination += obj3_ambient * light['ambient']
                if nearest_object == 3:
                    illumination += obj4_ambient * light['ambient']

            # diffuse
                if nearest_object == 0:
                    illumination += obj1_diffuse * light['diffuse'] * np.dot(intersection_to_light, normal_to_surface)
                if nearest_object == 1:
                    illumination += obj2_diffuse * light['diffuse'] * np.dot(intersection_to_light, normal_to_surface)
                if nearest_object == 2:
                    illumination += obj3_diffuse * light['diffuse'] * np.dot(intersection_to_light, normal_to_surface)
                if nearest_object == 3:
                    illumination += obj4_diffuse * light['diffuse'] * np.dot(intersection_to_light, normal_to_surface)
            # specular
                intersection_to_camera = normalize(camera[0] - intersection[0], camera[1] - intersection[1], camera[2] - intersection[2])
                H = normalize(intersection_to_light[0] + intersection_to_camera[0], intersection_to_light[1] + intersection_to_camera[1], intersection_to_light[2] + intersection_to_light[2])
                illumination += specular * light['specular'] * np.dot(normal_to_surface, H) ** (shinines / 4)

            # reflection
                color += reflection * illumination
                reflection *= obj_reflection

                origin = shifted_point
           
                dot_prod = dot_product(directionx, directiony, directionz, normal_to_surface[0], normal_to_surface[1], normal_to_surface[2])
                directionx = directionx - 2 * dot_prod * normal_to_surface[0]
                directiony = directiony - 2 * dot_prod * normal_to_surface[1]
                directionz = directionz - 2 * dot_prod * normal_to_surface[2]
            

        image[i, j] = np.clip(color, 0, 1)
    print("%d/%d" % (i + 1, height))

plt.imsave('image.png', image)