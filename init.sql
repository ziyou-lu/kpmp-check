drop table if exists kpmp_analysis_kdstaticface ;
CREATE TABLE kpmp_analysis_kdstaticface (
                                                   id serial4 NOT NULL,
                                                   faceid varchar(64) NULL,
                                                   shottime timestamptz NULL,
                                                   tabid varchar(128) NULL,
                                                   del int4 NULL,
                                                   featuredata varchar(1024) NULL,
                                                   CONSTRAINT kpmp_analysis_kdstaticface_pkey PRIMARY KEY (id)
);
CREATE INDEX kpmp_analysis_kdstaticface_faceid_idx ON kpmp_analysis_kdstaticface USING btree (faceid, shottime);
CREATE INDEX kpmp_analysis_kdstaticface_shottime_idx ON kpmp_analysis_kdstaticface USING btree (shottime);

drop table if exists kpmp_analysis_staticface ;
CREATE TABLE kpmp_analysis_staticface (
                                            id serial4 NOT NULL,
                                            faceid varchar(64) NULL,
                                            shottime timestamptz NULL,
                                            tabid varchar(128) NULL,
                                            del int4 NULL,
                                            CONSTRAINT kpmp_analysis_staticface_pkey PRIMARY KEY (id)
);
CREATE INDEX kpmp_analysis_staticface_faceid_idx ON kpmp_analysis_staticface USING btree (faceid, shottime);
CREATE INDEX kpmp_analysis_staticface_shottime_idx ON kpmp_analysis_staticface USING btree (shottime);

drop table if exists kpmp_data_ingestion_staticperson_input ;
CREATE TABLE kpmp_data_ingestion_staticperson_input (
                                          id serial4 NOT NULL,
                                          faceid varchar(64) NULL,
                                          shottime timestamptz NULL,
                                          tabid varchar(128) NULL,
                                          del int4 NULL,
                                          CONSTRAINT kpmp_data_ingestion_staticperson_input_staticface_pkey PRIMARY KEY (id)
);
CREATE INDEX kpmp_data_ingestion_staticperson_input_faceid_idx ON kpmp_data_ingestion_staticperson_input USING btree (faceid, shottime);
CREATE INDEX kpmp_data_ingestion_staticperson_input_shottime_idx ON kpmp_data_ingestion_staticperson_input USING btree (shottime);

drop table if exists kpmp_data_ingestion_staticface_input ;
CREATE TABLE kpmp_data_ingestion_staticface_input (
                                                        id serial4 NOT NULL,
                                                        faceid varchar(64) NULL,
                                                        shottime timestamptz NULL,
                                                        tabid varchar(128) NULL,
                                                        del int4 NULL,
                                                        CONSTRAINT kpmp_data_ingestion_staticface_input_staticface_pkey PRIMARY KEY (id)
);
CREATE INDEX kpmp_data_ingestion_staticface_input_faceid_idx ON kpmp_data_ingestion_staticface_input USING btree (faceid, shottime);
CREATE INDEX kpmp_data_ingestion_staticface_input_shottime_idx ON kpmp_data_ingestion_staticface_input USING btree (shottime);

drop table if exists kpmp_analysis_face ;
CREATE TABLE kpmp_analysis_face (
                                                      id serial4 NOT NULL,
                                                      faceid varchar(64) NULL,
                                                      shottime timestamptz NULL,
                                                      deviceid varchar(128) NULL,
                                                      del int4 NULL,
                                                      CONSTRAINT kpmp_analysis_face_staticface_pkey PRIMARY KEY (id)
);
CREATE INDEX kpmp_analysis_face_faceid_idx ON kpmp_analysis_face USING btree (faceid, shottime);
CREATE INDEX kpmp_analysis_face_shottime_idx ON kpmp_analysis_face USING btree (shottime);


drop table if exists kpmp_analysis_kdface ;
CREATE TABLE kpmp_analysis_kdface (
                                                      id serial4 NOT NULL,
                                                      faceid varchar(64) NULL,
                                                      shottime timestamptz NULL,
                                                      deviceid varchar(128) NULL,
                                                      featuredata varchar(2048) null,
                                                      del int4 NULL,
                                                      datapartition int NULL,
                                                      dataoffset bigint NULL,
                                                      CONSTRAINT kpmp_analysis_kdface_staticface_pkey PRIMARY KEY (id)
);
CREATE INDEX kpmp_analysis_kdface_faceid_idx ON kpmp_analysis_kdface USING btree (faceid, shottime);
CREATE INDEX kpmp_analysis_kdface_shottime_idx ON kpmp_analysis_kdface USING btree (shottime);

drop table if exists aias_kdface ;
CREATE TABLE aias_kdface (
                                    id serial4 NOT NULL,
                                    faceid varchar(64) NULL,
                                    shottime timestamptz NULL,
                                    imageid varchar(128) NULL,
                                    url varchar(1024) NULL,
                                    featuredata varchar(2048) NULL,
                                    del int4 NULL,
                                    CONSTRAINT aias_kdface_staticface_pkey PRIMARY KEY (id)
);
CREATE INDEX aias_kdface_faceid_only_idx ON aias_kdface USING btree (faceid);
CREATE INDEX aias_kdface_faceid_idx ON aias_kdface USING btree (faceid, shottime);
CREATE INDEX aias_kdface_shottime_idx ON aias_kdface USING btree (shottime);
CREATE INDEX aias_kdface_featuredata_idx ON aias_kdface USING btree (featuredata);