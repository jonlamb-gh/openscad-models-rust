$fn=50;
union()
{
	union()
	{
		translate([14.32,11.689999,0])
		{
			translate([3.81,3.81,0])
			{
				rotate(0,[0,0,1])
				{
					translate([-3.81,-3.81,0])
					{
						translate([7.62,0,0])
						{
							rotate(-90,[0,1,0])
							{
								difference()
								{
									color("Peru")
									{
										cube([81,7.62,7.62]);
									}
									translate([0,0,-0.1])
									{
										linear_extrude(height=7.8199997,center=false,convecity=10,twist=0,slices=1)
										{
											polygon(points=[[-0.1,-0.1],[-0.1,4.334375],[69.35,-0.1],],paths=undef,convexity=10);
										}
									}
									translate([0,-0.1,7.62])
									{
										rotate(-90,[1,0,0])
										{
											linear_extrude(height=7.8199997,center=false,convecity=10,twist=0,slices=1)
											{
												polygon(points=[[-0.1,-0.1],[-0.1,4.334375],[69.35,-0.1],],paths=undef,convexity=10);
											}
										}
									}
									translate([69.35,-0.1,2.86])
									{
										cube([10,7.8199997,1.9]);
									}
									translate([71.225,2.86,-0.1])
									{
										cube([6.25,1.9,7.8199997]);
									}
									translate([69.35,2.86,-0.1])
									{
										cube([6.25,1.9,3.81]);
									}
								}
							}
						}
					}
				}
			}
		}
		translate([14.32,81.69,0])
		{
			translate([3.81,3.81,0])
			{
				rotate(-90,[0,0,1])
				{
					translate([-3.81,-3.81,0])
					{
						translate([7.62,0,0])
						{
							rotate(-90,[0,1,0])
							{
								difference()
								{
									color("Peru")
									{
										cube([81,7.62,7.62]);
									}
									translate([0,0,-0.1])
									{
										linear_extrude(height=7.8199997,center=false,convecity=10,twist=0,slices=1)
										{
											polygon(points=[[-0.1,-0.1],[-0.1,4.334375],[69.35,-0.1],],paths=undef,convexity=10);
										}
									}
									translate([0,-0.1,7.62])
									{
										rotate(-90,[1,0,0])
										{
											linear_extrude(height=7.8199997,center=false,convecity=10,twist=0,slices=1)
											{
												polygon(points=[[-0.1,-0.1],[-0.1,4.334375],[69.35,-0.1],],paths=undef,convexity=10);
											}
										}
									}
									translate([69.35,2.86,-0.1])
									{
										cube([10,1.9,7.8199997]);
									}
									translate([71.225,-0.1,2.86])
									{
										cube([6.25,7.8199997,1.9]);
									}
									translate([69.35,3.81,2.86])
									{
										cube([6.25,7.62,1.9]);
									}
								}
							}
						}
					}
				}
			}
		}
		translate([93.06,81.69,0])
		{
			translate([3.81,3.81,0])
			{
				rotate(180,[0,0,1])
				{
					translate([-3.81,-3.81,0])
					{
						translate([7.62,0,0])
						{
							rotate(-90,[0,1,0])
							{
								difference()
								{
									color("Peru")
									{
										cube([81,7.62,7.62]);
									}
									translate([0,0,-0.1])
									{
										linear_extrude(height=7.8199997,center=false,convecity=10,twist=0,slices=1)
										{
											polygon(points=[[-0.1,-0.1],[-0.1,4.334375],[69.35,-0.1],],paths=undef,convexity=10);
										}
									}
									translate([0,-0.1,7.62])
									{
										rotate(-90,[1,0,0])
										{
											linear_extrude(height=7.8199997,center=false,convecity=10,twist=0,slices=1)
											{
												polygon(points=[[-0.1,-0.1],[-0.1,4.334375],[69.35,-0.1],],paths=undef,convexity=10);
											}
										}
									}
									translate([69.35,-0.1,2.86])
									{
										cube([10,7.8199997,1.9]);
									}
									translate([71.225,2.86,-0.1])
									{
										cube([6.25,1.9,7.8199997]);
									}
									translate([69.35,2.86,-0.1])
									{
										cube([6.25,1.9,3.81]);
									}
								}
							}
						}
					}
				}
			}
		}
		translate([93.06,11.689999,0])
		{
			translate([3.81,3.81,0])
			{
				rotate(90,[0,0,1])
				{
					translate([-3.81,-3.81,0])
					{
						translate([7.62,0,0])
						{
							rotate(-90,[0,1,0])
							{
								difference()
								{
									color("Peru")
									{
										cube([81,7.62,7.62]);
									}
									translate([0,0,-0.1])
									{
										linear_extrude(height=7.8199997,center=false,convecity=10,twist=0,slices=1)
										{
											polygon(points=[[-0.1,-0.1],[-0.1,4.334375],[69.35,-0.1],],paths=undef,convexity=10);
										}
									}
									translate([0,-0.1,7.62])
									{
										rotate(-90,[1,0,0])
										{
											linear_extrude(height=7.8199997,center=false,convecity=10,twist=0,slices=1)
											{
												polygon(points=[[-0.1,-0.1],[-0.1,4.334375],[69.35,-0.1],],paths=undef,convexity=10);
											}
										}
									}
									translate([69.35,2.86,-0.1])
									{
										cube([10,1.9,7.8199997]);
									}
									translate([71.225,-0.1,2.86])
									{
										cube([6.25,7.8199997,1.9]);
									}
									translate([69.35,3.81,2.86])
									{
										cube([6.25,7.62,1.9]);
									}
								}
							}
						}
					}
				}
			}
		}
	}
	translate([0,0,78.35])
	{
		union()
		{
			union()
			{
				translate([0,0,3.3])
				{
					difference()
					{
						color("SaddleBrown")
						{
							cube([115,25,3.3]);
						}
						translate([0,25.1,0])
						{
							rotate(90,[1,0,0])
							{
								linear_extrude(height=25.2,center=false,convecity=10,twist=0,slices=1)
								{
									polygon(points=[[-0.1,-0.1],[-0.1,1.875],[15,-0.1],],paths=undef,convexity=10);
								}
							}
						}
						translate([115,25,0])
						{
							rotate(180,[0,0,1])
							{
								translate([0,25.1,0])
								{
									rotate(90,[1,0,0])
									{
										linear_extrude(height=25.2,center=false,convecity=10,twist=0,slices=1)
										{
											polygon(points=[[-0.1,-0.1],[-0.1,1.875],[15,-0.1],],paths=undef,convexity=10);
										}
									}
								}
							}
						}
					}
				}
				translate([0,25,3.3])
				{
					difference()
					{
						color("SaddleBrown")
						{
							cube([115,13,3.3]);
						}
						translate([0,13.1,0])
						{
							rotate(90,[1,0,0])
							{
								linear_extrude(height=13.2,center=false,convecity=10,twist=0,slices=1)
								{
									polygon(points=[[-0.1,-0.1],[-0.1,1.875],[15,-0.1],],paths=undef,convexity=10);
								}
							}
						}
						translate([115,13,0])
						{
							rotate(180,[0,0,1])
							{
								translate([0,13.1,0])
								{
									rotate(90,[1,0,0])
									{
										linear_extrude(height=13.2,center=false,convecity=10,twist=0,slices=1)
										{
											polygon(points=[[-0.1,-0.1],[-0.1,1.875],[15,-0.1],],paths=undef,convexity=10);
										}
									}
								}
							}
						}
					}
				}
				translate([0,38,3.3])
				{
					difference()
					{
						color("SaddleBrown")
						{
							cube([115,25,3.3]);
						}
						translate([0,25.1,0])
						{
							rotate(90,[1,0,0])
							{
								linear_extrude(height=25.2,center=false,convecity=10,twist=0,slices=1)
								{
									polygon(points=[[-0.1,-0.1],[-0.1,1.875],[15,-0.1],],paths=undef,convexity=10);
								}
							}
						}
						translate([115,25,0])
						{
							rotate(180,[0,0,1])
							{
								translate([0,25.1,0])
								{
									rotate(90,[1,0,0])
									{
										linear_extrude(height=25.2,center=false,convecity=10,twist=0,slices=1)
										{
											polygon(points=[[-0.1,-0.1],[-0.1,1.875],[15,-0.1],],paths=undef,convexity=10);
										}
									}
								}
							}
						}
					}
				}
				translate([0,63,3.3])
				{
					difference()
					{
						color("SaddleBrown")
						{
							cube([115,13,3.3]);
						}
						translate([0,13.1,0])
						{
							rotate(90,[1,0,0])
							{
								linear_extrude(height=13.2,center=false,convecity=10,twist=0,slices=1)
								{
									polygon(points=[[-0.1,-0.1],[-0.1,1.875],[15,-0.1],],paths=undef,convexity=10);
								}
							}
						}
						translate([115,13,0])
						{
							rotate(180,[0,0,1])
							{
								translate([0,13.1,0])
								{
									rotate(90,[1,0,0])
									{
										linear_extrude(height=13.2,center=false,convecity=10,twist=0,slices=1)
										{
											polygon(points=[[-0.1,-0.1],[-0.1,1.875],[15,-0.1],],paths=undef,convexity=10);
										}
									}
								}
							}
						}
					}
				}
				translate([0,76,3.3])
				{
					difference()
					{
						color("SaddleBrown")
						{
							cube([115,25,3.3]);
						}
						translate([0,25.1,0])
						{
							rotate(90,[1,0,0])
							{
								linear_extrude(height=25.2,center=false,convecity=10,twist=0,slices=1)
								{
									polygon(points=[[-0.1,-0.1],[-0.1,1.875],[15,-0.1],],paths=undef,convexity=10);
								}
							}
						}
						translate([115,25,0])
						{
							rotate(180,[0,0,1])
							{
								translate([0,25.1,0])
								{
									rotate(90,[1,0,0])
									{
										linear_extrude(height=25.2,center=false,convecity=10,twist=0,slices=1)
										{
											polygon(points=[[-0.1,-0.1],[-0.1,1.875],[15,-0.1],],paths=undef,convexity=10);
										}
									}
								}
							}
						}
					}
				}
			}
			union()
			{
				translate([21.940002,9,0])
				{
					translate([0,83,0])
					{
						rotate(-90,[0,0,1])
						{
							color("SandyBrown")
							{
								cube([83,5,3.3]);
							}
						}
					}
				}
				translate([88.06,9,0])
				{
					translate([0,83,0])
					{
						rotate(-90,[0,0,1])
						{
							color("SandyBrown")
							{
								cube([83,5,3.3]);
							}
						}
					}
				}
			}
		}
	}
	union()
	{
		translate([11.779999,14.549999,69.35])
		{
			translate([0,1.9,0])
			{
				rotate(90,[1,0,0])
				{
					difference()
					{
						color("BurlyWood")
						{
							cube([91.44,10,1.9]);
						}
						union()
						{
							translate([-0.1,-0.1,-0.1])
							{
								cube([7.4000006,1.975,2.1]);
							}
							translate([0,8.225,0])
							{
								translate([-0.1,-0.1,-0.1])
								{
									cube([10.26,1.975,2.1]);
								}
							}
						}
						translate([84.24,0,0])
						{
							union()
							{
								translate([-0.1,-0.1,-0.1])
								{
									cube([7.4000006,1.975,2.1]);
								}
								translate([-2.86,8.225,0])
								{
									translate([-0.1,-0.1,-0.1])
									{
										cube([10.26,1.975,2.1]);
									}
								}
							}
						}
						translate([1.9399999,4.05,-0.1])
						{
							cube([0.6,1.9,2.1]);
						}
						translate([88.9,4.05,-0.1])
						{
							cube([0.6,1.9,2.1]);
						}
					}
				}
			}
		}
		translate([11.779999,84.55,69.35])
		{
			translate([0,1.9,0])
			{
				rotate(90,[1,0,0])
				{
					difference()
					{
						color("BurlyWood")
						{
							cube([91.44,10,1.9]);
						}
						union()
						{
							translate([-0.1,-0.1,-0.1])
							{
								cube([7.4000006,1.975,2.1]);
							}
							translate([0,8.225,0])
							{
								translate([-0.1,-0.1,-0.1])
								{
									cube([10.26,1.975,2.1]);
								}
							}
						}
						translate([84.24,0,0])
						{
							union()
							{
								translate([-0.1,-0.1,-0.1])
								{
									cube([7.4000006,1.975,2.1]);
								}
								translate([-2.86,8.225,0])
								{
									translate([-0.1,-0.1,-0.1])
									{
										cube([10.26,1.975,2.1]);
									}
								}
							}
						}
						translate([1.9399999,4.05,-0.1])
						{
							cube([0.6,1.9,2.1]);
						}
						translate([88.9,4.05,-0.1])
						{
							cube([0.6,1.9,2.1]);
						}
					}
				}
			}
		}
		translate([17.18,10.689999,69.35])
		{
			translate([1.9,79.62,0])
			{
				rotate(-90,[0,1,0])
				{
					rotate(-90,[0,0,1])
					{
						difference()
						{
							color("BurlyWood")
							{
								cube([79.62,10,1.9]);
							}
							translate([3.86,1.875,-0.1])
							{
								cube([1.9,6.25,2.1]);
							}
							translate([73.86001,1.875,-0.1])
							{
								cube([1.9,6.25,2.1]);
							}
						}
					}
				}
			}
		}
		translate([95.92,10.689999,69.35])
		{
			translate([1.9,79.62,0])
			{
				rotate(-90,[0,1,0])
				{
					rotate(-90,[0,0,1])
					{
						difference()
						{
							color("BurlyWood")
							{
								cube([79.62,10,1.9]);
							}
							translate([3.86,1.875,-0.1])
							{
								cube([1.9,6.25,2.1]);
							}
							translate([73.86001,1.875,-0.1])
							{
								cube([1.9,6.25,2.1]);
							}
						}
					}
				}
			}
		}
	}
	union()
	{
		translate([13.720001,12.689999,73.4])
		{
			rotate(90,[0,1,0])
			{
				rotate(90,[0,0,1])
				{
					color("SaddleBrown")
					{
						cube([6,1.9,0.6]);
					}
				}
			}
		}
		translate([13.720001,82.5,73.4])
		{
			rotate(90,[0,1,0])
			{
				rotate(90,[0,0,1])
				{
					color("SaddleBrown")
					{
						cube([6,1.9,0.6]);
					}
				}
			}
		}
		translate([100.68,82.5,73.4])
		{
			rotate(90,[0,1,0])
			{
				rotate(90,[0,0,1])
				{
					color("SaddleBrown")
					{
						cube([6,1.9,0.6]);
					}
				}
			}
		}
		translate([100.68,12.689999,73.4])
		{
			rotate(90,[0,1,0])
			{
				rotate(90,[0,0,1])
				{
					color("SaddleBrown")
					{
						cube([6,1.9,0.6]);
					}
				}
			}
		}
	}
}
