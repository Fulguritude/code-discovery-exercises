from __future__ import annotations
from typing     import TypedDict
from functools  import reduce

from   torch import Tensor
import torch
import torchvision.transforms as transforms

from image_io import ImageTensor



# This is a type used to store an image's metadata, as required
# per the exercise.
ImageInfo = TypedDict(
	"ImageInfo",
	{
		"Filename"      : str,
		"Height"        : float,
		"Width"         : float,
		"Average_Red"   : float,
		"Average_Green" : float,
		"Average_Blue"  : float,
	},
	total = True,
)


def fold_tensor_on_width_and_height(
	filename : str,
	tensor   : ImageTensor,
) -> ImageInfo:
	# get image metadata
	_, height, width = tensor.shape
	pixel_amount     = height * width

	#compute averages by folding along width then height
	inv_pixel_amount = 1. / pixel_amount
	tensor = torch.sum(tensor, dim = 2)
	tensor = torch.sum(tensor, dim = 1)
	r_avg = float(tensor[0] * inv_pixel_amount)
	g_avg = float(tensor[1] * inv_pixel_amount)
	b_avg = float(tensor[2] * inv_pixel_amount)

	#package result neatly
	result : ImageInfo = {
		"Filename"      : filename,
		"Height"        : height,
		"Width"         : width,
		"Average_Red"   : r_avg,
		"Average_Green" : g_avg,
		"Average_Blue"  : b_avg,
	}
	return result
