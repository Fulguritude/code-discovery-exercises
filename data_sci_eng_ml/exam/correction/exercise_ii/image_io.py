from __future__ import annotations
from typing import Optional
import glob

from   PIL   import Image
from   torch import Tensor
import torch
import torchvision.transforms as transforms #type: ignore



# This is a type alias to signify that this should be a rank 3 tensor with RGB-Width-Height dims
ImageTensor = Tensor

def read_image(filepath: str) -> Image.Image:
	# this function is a 'useless' wrapper (just a rename) over Image.open
	result = Image.open(filepath)
	return result


def tensor_from_pil(pil_img: Image.Image) -> ImageTensor:
	# this function converts a PIL Image into an ImageTensor, using torch
	transform = transforms.Compose(
		[
			transforms.PILToTensor()
		]
	)
	result : Tensor = transform(pil_img)  #type: ignore
	return result


def read_imagetensor(filepath: str) -> ImageTensor:
	image  = read_image(filepath)
	tensor = tensor_from_pil(image)
	image.close()
	return tensor


def read_image_dataset(
	dir_filepath : str,
	limit        : Optional[int] = None
) -> dict[str, ImageTensor]:
	# this function reads all image files in a directory
	filenames = glob.glob(dir_filepath + "*")
	filenames.sort()
	filenames = filenames if limit is None else filenames[:limit]
	tensors   = {
		filename: read_imagetensor(filename)
		for filename in filenames
	}
	return tensors
