drop table if exists kpmp_analysis_kdstaticface ;
create table kpmp_analysis_kdstaticface (
	id SERIAL primary key,
	FaceID varchar(64),
	ShotTime date,
	FeatureData varchar(1024),
	del int
);