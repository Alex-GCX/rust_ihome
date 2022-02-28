from ihome.models import Areas
from ihome import db

area1=Areas(name='东城区')
area2=Areas(name='西城区')
area3=Areas(name='朝阳区')
area4=Areas(name='海淀区')
area5=Areas(name='昌平区')
area6=Areas(name='丰台区')
area7=Areas(name='房山区')
area8=Areas(name='通州区')
area9=Areas(name='顺义区')
area10=Areas(name='大兴区')
area11=Areas(name='怀柔区')
area12=Areas(name='平谷区')
area13=Areas(name='密云区')
area14=Areas(name='延庆区')
area15=Areas(name='石景山区')
area16=Areas(name='门头沟区')

db.session.add_all([area1,area2,area3,area4,area5,area6,area7,area8,area9,area10,area11,area12,area13,area14,area15,area16])
db.session.commit()
