from ihome.models import Facilities
from ihome import db

faci1=Facilities(name='无线网络')
faci2=Facilities(name='热水淋浴')
faci3=Facilities(name='空调')
faci4=Facilities(name='暖气')
faci5=Facilities(name='允许吸烟')
faci6=Facilities(name='饮水设备')
faci7=Facilities(name='牙具')
faci8=Facilities(name='香皂')
faci9=Facilities(name='拖鞋')
faci10=Facilities(name='手纸')
faci11=Facilities(name='毛巾')
faci12=Facilities(name='沐浴露、洗发露')
faci13=Facilities(name='冰箱')
faci14=Facilities(name='洗衣机')
faci15=Facilities(name='电梯')
faci16=Facilities(name='允许做饭')
faci17=Facilities(name='允许带宠物')
faci18=Facilities(name='允许聚会')
faci19=Facilities(name='门禁系统')
faci20=Facilities(name='停车位')
faci21=Facilities(name='有线网络')
faci22=Facilities(name='电视')
faci23=Facilities(name='浴缸')

db.session.add_all([faci1,faci2,faci3,faci4,faci5,faci6,faci7,faci8,faci9,faci10,faci11,faci12,faci13,faci14,faci15,faci16,faci17,faci18,faci19,faci20,faci21,faci22,faci23])
db.session.commit()

# INSERT INTO `ih_facilities`(`name`) VALUES('无线网络'),('热水淋浴'),('空调'),('暖气'),('允许吸烟'),('饮水设备'),('牙具'),('香皂'),('拖鞋'),('手纸'),('毛巾'),
# ('沐浴露、洗发露'),('冰箱'),('洗衣机'),('电梯'),('允许做饭'),('允许带宠物'),('允许聚会'),('门禁系统'),('停车位'),('有线网络'),('电视'),('浴缸');