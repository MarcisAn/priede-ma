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

def normalize_x(x,y,z):
    xnorm = x / sqrt(x**2 + y**2 + z**2)
    
    return xnorm

def normalize_y(x,y,z):
    ynorm = y / sqrt(x**2 + y**2 + z**2)
    
    return ynorm

def normalize_z(x,y,z):
    znorm = z / sqrt(x**2 + y**2 + z**2)
    
    return znorm

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

def does_intersect(orig_x, orig_y, orig_z, dir_x, dir_y, dir_z):
    inter0 = do_spheres_intersect(obj1_center_x, obj1_center_y, obj1_center_z, obj1_rad, orig_x, orig_y, orig_z, dir_x, dir_y, dir_z)
    inter1 = do_spheres_intersect(obj2_center_x, obj2_center_y, obj2_center_z, obj2_rad, orig_x, orig_y, orig_z, dir_x, dir_y, dir_z)
    inter2 = do_spheres_intersect(obj3_center_x, obj3_center_y, obj3_center_z, obj3_rad, orig_x, orig_y, orig_z, dir_x, dir_y, dir_z)
    inter3 = do_spheres_intersect(obj4_center_x, obj4_center_y, obj4_center_z, obj4_rad, orig_x, orig_y, orig_z, dir_x, dir_y, dir_z)
    if inter0 or inter1 or inter2 or inter3:
        return True
    else:
        return False

def nearest_intersected_object(orig_x, orig_y, orig_z, dir_x, dir_y, dir_z):
    distance0 = sphere_intersect(obj1_center_x, obj1_center_y, obj1_center_z, obj1_rad, orig_x, orig_y, orig_z, dir_x, dir_y, dir_z)
    distance1 = sphere_intersect(obj2_center_x, obj2_center_y, obj2_center_z, obj2_rad, orig_x, orig_y, orig_z, dir_x, dir_y, dir_z)
    distance2 = sphere_intersect(obj3_center_x, obj3_center_y, obj3_center_z, obj3_rad, orig_x, orig_y, orig_z, dir_x, dir_y, dir_z)
    distance3 = sphere_intersect(obj4_center_x, obj4_center_y, obj4_center_z, obj4_rad, orig_x, orig_y, orig_z, dir_x, dir_y, dir_z)
    min_distance = 1000000000000.0
    
    inter0 = do_spheres_intersect(obj1_center_x, obj1_center_y, obj1_center_z, obj1_rad, orig_x, orig_y, orig_z, dir_x, dir_y, dir_z)
    inter1 = do_spheres_intersect(obj2_center_x, obj2_center_y, obj2_center_z, obj2_rad, orig_x, orig_y, orig_z, dir_x, dir_y, dir_z)
    inter2 = do_spheres_intersect(obj3_center_x, obj3_center_y, obj3_center_z, obj3_rad, orig_x, orig_y, orig_z, dir_x, dir_y, dir_z)
    inter3 = do_spheres_intersect(obj4_center_x, obj4_center_y, obj4_center_z, obj4_rad, orig_x, orig_y, orig_z, dir_x, dir_y, dir_z)
   
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


light_pos = 5
light_ambient = 1
light_diffuse = 1
light_specular = 1

image = np.zeros((height, width, 3))
for i, y in enumerate(np.linspace(screen[1], screen[3], height)):
    for j, x in enumerate(np.linspace(screen[0], screen[2], width)):
        # screen is on origin
        pixel = np.array([x, y, 0])
        origin_x = 0
        origin_y = 0
        origin_z = 1
        tonormx= pixel[0] - origin_x
        tonormy = pixel[1] - origin_y
        tonormz = pixel[2] - origin_z
        
        directionx = tonormx / sqrt(tonormx**2 + tonormy**2 + tonormz**2)
        directiony = tonormy / sqrt(tonormx**2 + tonormy**2 + tonormz**2)
        directionz = tonormz / sqrt(tonormx**2 + tonormy**2 + tonormz**2)
        

        color = np.zeros((3))
        reflection = 1

        for k in range(max_depth):
            # check for intersections
            if does_intersect(origin_x, origin_y, origin_z, directionx, directiony, directionz):
                nearest_object, min_distance = nearest_intersected_object(origin_x, origin_y, origin_z, directionx, directiony, directionz)
                if nearest_object is None:
                   break

                intersection_x = origin_x + min_distance * directionx
                intersection_y = origin_y + min_distance * directiony
                intersection_z = origin_z + min_distance * directionz
            
                if nearest_object == 0: 
                    to_norm_x = intersection_x - obj1_center_x
                    to_norm_y = intersection_y - obj1_center_y
                    to_norm_z = intersection_z - obj1_center_z
                
                if nearest_object == 1: 
                    to_norm_x = intersection_x - obj2_center_x
                    to_norm_y = intersection_y - obj2_center_y
                    to_norm_z = intersection_z - obj2_center_z
                
                if nearest_object == 2: 
                    to_norm_x = intersection_x - obj3_center_x
                    to_norm_y = intersection_y - obj3_center_y
                    to_norm_z = intersection_z - obj3_center_z
                
                if nearest_object == 3: 
                    to_norm_x = intersection_x - obj4_center_x
                    to_norm_y = intersection_y - obj4_center_y
                    to_norm_z = intersection_z - obj4_center_z
            
                normx = to_norm_x / sqrt(to_norm_x**2 + to_norm_y**2 + to_norm_z**2)
                normy = to_norm_y / sqrt(to_norm_x**2 + to_norm_y**2 + to_norm_z**2)
                normz = to_norm_z / sqrt(to_norm_x**2 + to_norm_y**2 + to_norm_z**2)
            
                
                shifted_point_x = intersection_x + 1e-5 * normx
                shifted_point_y = intersection_y + 1e-5 * normy
                shifted_point_z = intersection_z + 1e-5 * normz
                
                intersection_to_light_x = normalize_x(light_pos - shifted_point_x, light_pos - shifted_point_y,light_pos - shifted_point_z)
                intersection_to_light_y = normalize_y(light_pos - shifted_point_x, light_pos - shifted_point_y,light_pos - shifted_point_z)
                intersection_to_light_z = normalize_z(light_pos - shifted_point_x, light_pos - shifted_point_y,light_pos - shifted_point_z)
                
                _, min_distance = nearest_intersected_object(shifted_point_x, shifted_point_y, shifted_point_z, intersection_to_light_x, intersection_to_light_y, intersection_to_light_z)
                intersection_to_light_distance = sqrt((light_pos - shifted_point_x)**2 + (light_pos - shifted_point_y)**2 + (light_pos - shifted_point_z)**2)
                is_shadowed = min_distance < intersection_to_light_distance

                if is_shadowed:
                    break

                illumination = np.zeros((3))

            # ambiant
                if nearest_object == 0:
                    illumination += obj1_ambient * light_ambient
                if nearest_object == 1:
                    illumination += obj2_ambient * light_ambient
                if nearest_object == 2:
                    illumination += obj3_ambient * light_ambient
                if nearest_object == 3:
                    illumination += obj4_ambient * light_ambient

            # diffuse
                if nearest_object == 0:
                    illumination += obj1_diffuse * light_diffuse * dot_product(intersection_to_light_x, intersection_to_light_y, intersection_to_light_z, normx, normy, normz)
                if nearest_object == 1:
                    illumination += obj2_diffuse * light_diffuse * dot_product(intersection_to_light_x, intersection_to_light_y, intersection_to_light_z, normx, normy, normz)
                if nearest_object == 2:
                    illumination += obj3_diffuse * light_diffuse * dot_product(intersection_to_light_x, intersection_to_light_y, intersection_to_light_z, normx, normy, normz)
                if nearest_object == 3:
                    illumination += obj4_diffuse * light_diffuse * dot_product(intersection_to_light_x, intersection_to_light_y, intersection_to_light_z, normx, normy, normz)
            # specular
                intersection_to_camera_x = normalize_z(0 - intersection_x, 0 - intersection_y, 1 - intersection_z)
                intersection_to_camera_y = normalize_y(0 - intersection_x, 0 - intersection_y, 1 - intersection_z)
                intersection_to_camera_z = normalize_z(0 - intersection_x, 0 - intersection_y, 1 - intersection_z)
                
                H_x = normalize_x(intersection_to_light_x + intersection_to_camera_x, intersection_to_light_y + intersection_to_camera_y, intersection_to_light_z + intersection_to_camera_z)
                H_y = normalize_y(intersection_to_light_x + intersection_to_camera_x, intersection_to_light_y + intersection_to_camera_y, intersection_to_light_z + intersection_to_camera_z)
                H_z = normalize_z(intersection_to_light_x + intersection_to_camera_x, intersection_to_light_y + intersection_to_camera_y, intersection_to_light_z + intersection_to_camera_z)
                illumination += specular * light_specular * dot_product(normx, normy, normz, H_x, H_y, H_z) ** (shinines / 4)

            # reflection
                color += reflection * illumination
                reflection *= obj_reflection

                origin_x = shifted_point_x
                origin_y = shifted_point_y
                origin_z = shifted_point_z
           
                dot_prod = dot_product(directionx, directiony, directionz, normx, normy, normz)
                directionx = directionx - 2 * dot_prod * normx
                directiony = directiony - 2 * dot_prod * normy
                directionz = directionz - 2 * dot_prod * normz
            

        image[i, j] = np.clip(color, 0, 1)
    print("%d/%d" % (i + 1, height))

plt.imsave('image.png', image)