use crate::solver::Solver;

#[derive(Default)]
pub struct Solution {
    calorie_counts: Vec<u32>,
}

impl Solver for Solution {
    fn get_input(&self) -> &'static str {
        INPUT
    }

    fn with_input(&mut self, input: &str) {
        let mut calorie_counts =
            input
                .split('\n')
                .map(|l| l.parse::<u32>().ok())
                .fold(vec![0], |mut acc, n| {
                    if let Some(n) = n {
                        let total_calories = acc.last_mut().unwrap();
                        *total_calories += n;
                    } else {
                        acc.push(0);
                    }
                    acc
                });

        calorie_counts.sort();
        calorie_counts.reverse();
        self.calorie_counts = calorie_counts;
    }

    fn solve_part1(&self) -> String {
        self.calorie_counts.first().unwrap().to_string()
    }

    fn solve_part2(&self) -> String {
        self.calorie_counts[0..3].iter().sum::<u32>().to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_input<'a>() -> &'a str {
        "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"
    }

    #[test]
    fn test_solution_example1() {
        let mut solver = Solution::default();
        solver.with_input(get_input());
        let solution = solver.solve_part1();
        assert_eq!(solution, "24000");
    }

    #[test]
    fn test_solution_example2() {
        let mut solver = Solution::default();
        solver.with_input(get_input());
        let solution = solver.solve_part2();
        assert_eq!(solution, "45000");
    }
}

const INPUT: &str = "\
2027
1630
4699
3860
5686
1178
4983
1075
5436
2522
4455
4808
3644
2344
1671

45637

5634
2755
3537
5047
1878
4820
5959
5412
6458
1544
3606
4374
2743
3946

3013
1604
1574
3617
5992
7090
3693
7543
3619
5161
1137

2090
4120
5062
5286
5991
5770
1832
5860
4065
3299
4858
2220
3652
1601
2971

1099
4518
4639
5490
3661
1338
1250
1637
4865
4142
6223
3889
3586

8689
2655
3240
4869
2797
2615
9250
8800
3489

7077
11230
3357
8067
7843
2101

6938
4547
4785
1821
1191
3115
1608
1644
6265
2912
3866
3085
6350

6933
7966
5328
2300
1691
3347
3554
3506
5778
4815
6384

1869
8526
6994
4518
9372
8359
1354
1593
9572

7438
6028
7284
4254
5602
4651
4459
3257
1556
6858
5888

2934
5783
2790
8581
7143
5586
3408
9516
8532

8606
9150
7850
9817
8995
3645
7248

3670
5035
9209
9281
4264
11006
10744

1884
3682
4911
2095
2973
2384
2921
1847
1164
4809
5271
2540
3199
2248

3565
14021

4689
1709
4025
1640
8188
5901
8131
5368
7667
5701

1292
6767
2492
5091
5648
4131
5723
6690
4779
5389
6123

2943
6785
4572
4216
1685
6647
6254
5145
6518
1092
6018
6578
3802

6074
3735
5503
6045
1125
1936
1492
2109
3276
2303
3694
4043
1914
4098
3215

4485
9863
1350
9216
10090
2706
3036
4765

60052

27994
10519

15013
9391
6827
9619

3933
5536
4639
4932
6367
5494
6313
5941
5778
5737
2327
6004
5653
2110

22617
13105
13122

8004
10591
2884
6652
3057
4108
6168

4749
6610
4720
6131
2018
2377
4956
2644
3541
2238
5240
6349
5505

1037
3252
4898
2024
8504
7033
2874
6128
1980

4699
3306
3734
2837
1783
3867
5429
3508
6014
3050
6012
3899
1027
4580
4325

2206
3154
3199
5827
6462
6192
4555
7211
3711
4878
1428

33259

2777
8169
1498
1177
6185
7142
1774
2608
2972
1709

2410
10016
15706
9020
6509

10963
8354
8340
6759

5319
1256
2186
4087
3519
7098
1081
6427
1521
4709
2746
4891

2738
11494
1769
8394
11712

65573

3664
1231
5854
4212
2596
6327
4390
2409
6116
2561
4429

24315

5567
7242
5682
1615
7493
7470
9403
3017
1771

20069
7742
12802
1493

2013
2297
2324
6039
7389
5114
2682
1707
5585
1237
2260
4858

6017
5726
6355
4942
3158
4415
3346
1463
3923
1209
2913
5037
2016
6064

7098
1917
10007
7826
3511
5133
11079

5649
3881
6297
4598
2432
5596
2415
1486
3818
6301
5745
6471
1244
4187

1210
1041
4046
2045
10697
2330
9004
10726

4377
2131
7002
2683
2687
3402
4543
4175
6869
6090
2786
6047

1832
1034
4620
9711
4342
3099
10976

7148
1987
1931
2621
2633
7127
2604
2622
4548
6189
2600

3957
3819
7074
6150
7606
3992
6821
5095
2309
5017

6901

8398
17323
19438

5947
2945
5633
5800
6460
1355
2012
1447
2149
5744
6200
2639
4355
1968

5538
4251
5892
6830
1762
1036
3612
6244
4782
6287
1931

2348
2843
2432
4443
6494
2005
3571
1806
3730
2017
2756
4542
3344
5201

10804
14245
10498
6886
5008

5027
2158
7789
2715
5572
7146
7447
3866
6204
6891
4435

6925
6285
4040
7267
1100
1047
5065
6767
5364
3355
3027

5462
5112
3869
7812
7417
6774
7515
3704
3249
4121
4968

3350
6479
5526
4941
1778
4231
5206
6413
5345
1636
3700
1272
4754
1912

22244
3198
21060

4332
4443
3663
6023
1561
2464
1298
4716
5240
1185
1160
1335
1443
1390
1199

2922
4259
4428
6683
1393
2906
5748
5864
2095
5670
3024
1872

2234
1405
3544
1723
6248
3341
5181
5218
1221
6879
1497
3410

1946
2105
1134
4521
3563
2700
4447
3817
4872
5061
2599
6906
5192

17189
28261

3907
19712
8304
1893

28622
31375

7930
10163
6353
7503
1718
4393
8466
10328

7849
5028
2756
5881
1170
10304
10881

1235
4065
2126
3126
5312
4690
3678
4840
4654
5302
2613
2613
2104
2300

1891
1404
7285
13044

6602
6004
6908
2256
7470
2649
4304
2174
5771
6100
4847
5478

7900
3208
4516
3946
8080
9521
6260
2521
2390

9535
5121
6699
1021
2690
4732
8809
3674

2791
6279
1800
3714
1704
2323
6465
3359
8967

5562
5073
5435
5111
3168
4515
5547
3209
5793
5797
5566
4357
2733
1171
1936

6173
3069
2069
3452
4352
1218
2735
3813
4213
5945
5205
3454
3088
5621

12866
6027
4568
3670
10417
13811

11627
2897
2185
15991

4464
2760
3268
5727
3227
5925
3029
4196
5088
2284
2524
5533
6102
4655
4313

16252
1534
15202
14492
14613

6415
3700
4798
1156
1448
3564
4409
6529
4300
3702
4385
4374

5434
7019
8053
2165
5740
6296
5773
7548
4915
3054
4701

4468
1032
3009
4402
5898
4332
7234
4796
4724
5922
7658

4025
3389
4661
9813
4123
3049
1309
7315

10317
21781
2954

47700

23878
17812
23563

4868
4048
3359
1235
3164
5360
4233
2979
2448
5409
1781
5887
4374
5203
4575

54093

1118
4593
15552

4427
2040
3959
5155
9232
8601
6036
2782
8406

8734
6568
5269
7817
1965
7999
1995

13098
18529
18756
1395

3509
3960
13008
6825
4375
3112

5741
7233
3924
8065
6985
3713
2411
6838
6702
2429
1317

4100
3993
4277
4020
4741
4797
3603
4115
2975
3539
1104
4463
1954
3029
3294

52594

3938
6630
2893
7835
1157
7407
1833
5657
8753
6753

10319
7794
15576

8002
11552
3812
8899
13328

3893
3475
4265
1980
2801
3861
3143
3647
1350
5105
3567
3174
3112
6022
2392

7164
9431
6690
4101
10412
5285
3969

12063
3626
11020
10550
4411
6723

1176
4366
8927
10363
9953
7791
2469

10006
4788
5161
9005
7028
6069
4417

5894
2368
1360
4042
2032
4923
4759
3793
1221
1408
3814
5027
5886
5024
5322

4201
7866
5935
1390
2539
6863
5422
3098
4309
7759
1436

10513
7767
6016
7465
2586
3735
1617
10519

1664
5437
6530
5986
5379
2631
7333
1158
7465
6119
2027
2757

11966
16125
5412
13768
2875

3045
1986
6221
1155
2462
6308
1777
4252
5599
3283
4440
4030
4611
2189

8665
5185
7332
1602
4906
8201
10534
1250

9414
15988
20259

16376
35216

10504
10559
3453
2555
8031
8171
5652
1808

6385
8768
2810
2277
3799
7156
2628
4896
1157

1404
1989
1041
6817
6293
1266
4749
5135
4202
1972
5467
1025
4415

7368
3084
10667
1304
8356
3870
11396

23823
14273

2760
8218
1757
3202
2184
1349
4998
8279
6174

1621
5236
1781
5841
1898
4158
1564
6037
6008
1395
3487
3376
1642

2635
14067
4163
2353

4013
3969
1457
3685
6313
1670
5996
4196
1123
2670
5062
3883
5623
5384

19520
11543
9673
3236

5277
5799
5616
1842
2631
5400
5678
7169
8106
8568

4386
5190
6047
1134
2853
3514
3013
1308
2596
4524
2550
5740
2373
3345
4160

6477
2486
7904
11740
7511
5658

4114
5652
5800
3553
1502
6544
2502
8027
3478
5784
2985

6203
1685
3864
6233
8073
7063
3644
8634

8460
10715
9641
4407
7126
8160
2670
5507

7020
3413
1725
2459
2496
3566
6845
6503
2660
6354
2879
1880

5702
3998
4365
5908
4970
2515
4611
3791
6810
3870
3043
2930
4526

2044
2687
6635
4412
3605
1794
2261
7638
2825
6429
8056

4540
12136
10484
8080
8184

3236
1709
3399
3589
4307
7279
6386
4565
3014
4595
5491
2191

25320
22508
12693

6912
10857
3909

2539
4150
4884
2280
2554
2669
2175
2976
2493
4886
5329
1175
2723
6024
5226

5427
1509
1722
7642
1370
4273
3778
4726
2547
1336
7620

8093
11419
4539
7336
16443

19941

1205
3022
3906
6829
1184
5122
1201
6197
1479
2753
6054
6115
2521

8154
1042
6337
1204
5113
8594
1717
2834
3145
6275

3652
6449
8325
2696
6357
4185
1021
8122
3606

1231
36594

6583
2371
4792
8445
13230
9283

6198
3914
2163
1632
6095
4101
3091
1123
2247
6283
2128
2844
1500
5219

3684
2056
3058
4540
4612
1097
5224
3276
3598
5218
3357
5942
4746
3544

4969
3867
2537
5619
1424
4564
3278
5945
2096
3786
5507
4954
4256
1883
2308

5299
4588
1992
4631
6546
6547
3615
5492
1454
1887
6150
1251
1768

6631
11717
1876
4872
4428
9330
6908

6222
5418
6155
1488
1607
4398
1240
6790
6576
6394
1226
2137
2434

1219
1531
5677
1759
6053
5517
4208
4807
5350
4145
4783
2138
2163
3865
3373

1718
4607
5691
5819
5049
4711
3849
5074
2344
3803
5898
5517
4699
2241
1834

1288
5799
6209
3306
3942
1017
4983
2772
5992
2985
5708
1804
5353

1144
5630
4767
2205
4815
1667
6046
3517
4125
2404
4803
3395
3046
3098
3115

7279
8793
3168
7768
4294
2290
1856
2877
5003
1985

6309
1134
2680
4767
2372
3941
3175
5395
6224
2641
2978
2670

9595
7700
2871
6777
4880
6013
2954
2282
1637

48865

3111
1159
3538
6881
6182
5496
5489
2871
2354
4782
1677
3505

4516
3376
2201
6140
5269
5559
1615
4646
5534
5324
2923
1087
2771
5849

1975
2516
4936
1235
5770
1754
2957
5212
5975
3092
4532
3367
2339
4255
3518

4924
7127
8605
9681
9072
5285
7430
8392
4883

12466
10398
9835
12600
11072
4934

4289
3361
1307
4107
6558
1763
4028
6989
4517
2628
1769
4000

3936
4424
1897
4671
5832
3001
5135
2954
4288
3371
5637
3535
4382
1047

12798
7554
7324
6381
4750
2463

3349
3341
5990
7280
1956
5788
6985
2406
6386
2595
1233

1329
4698
4132
2743
1719
5819
1507
3826
3926
2113
5850
2226
2974
3406
2518

7686
9278
8010
7102
6317
1041
4989
8136
7636

20603
24988
15524

11697
15179
25841

2109
5575
5758
2803
8133
6313
3869
4283
3620
4145

33038
34530

3379
2381
2594
3734
5163
2156
4421
4251
2246
1383
4847
4720
4789
4855

5355
5571

12519
6570
2892
12707
6357

4243
3435
2209
3438
1990
4060
4638
6062
3367
4648
3484

3570
3708
1672
2074
5067
2469
1763
3231
4156
5631
5352
1282
4325
1986

2907
5467
8206
4394
2961
3848
7586
7282

1455
5104
5480
2317
4167
4499
1640
3278
4009
1767
5765
1905
2709
5366
5081

14518
5554
15745
16239
12430

8089
9041
8911
8130
4466
5544
11913

9939
8050
14954
10256

1196
3418
6172
4525
3323
3696
4426
4253
5986
7078
6267

5972
2908
3125
6348
1165
2848
2775
1528
2013
3587
5002
2171
1295
6266

6496
6113
6011
5648
1478
5789
3842
6380
1032
1636
4925
2313
3948
3187

8982
1452
3024
10386
10158
4211
5666

1700
8961
2116
2313
1360
5776
8830
4218
5986

13542
15200
5418
9755

1092
6267
5903
7475
5271
5971
3886
3599
6591
4197
4863
3366

5754
5032
1227
1538
1051
5823
5542
6875
3150
4597
6285
5140
6136

36140

6408
3234
6142
7311
3370
7540
8218
1106
4074
5224

16367
9442
5310

1768
10067
2701
3965
5472
4463
6093
5819

15941
13948
6848

12952

5060
3640
5851
2513
1530
1382
2215
5755
1973
2325
4124
6885

6380
6785
6412
1546
5424
5415
3007
8822
1056

3254
1738
4573
1822
4519
1247
5048
4302
2892
5403
4227
4931
3821
4269
3872

2667
5833
1830
5667
4975
4814
5370
2245
4402
7131
4758
4971

8751
1587
1960
9338
3976
3569
7813
9498
8921

1023
3250
1680
5896
3925
4139
3074
1168
2859
4303
4546

2311
1043
5507
10776
2024
11480

6402
4395
1527
5462
3469
6359
1591
4790
1589
2905
2576
7161

5711
2973
7237
1373
5758
7209
5920
6021
7143
1699
6420

2312
7473
11511
10062
7851
1170
5633

8730
7260
3971
1720
5471
7552
4755
4039
7625
6259

4708
4366
6162
5650
4547
6362
2820
1727
6011
3535
1046
4966
1985
4650

2915
3837
5647
4514
1168
2917
5179
3861
5770
5321
3036
5190
4409
1656
1633

8649
5417
6918
2414
5823
1550
10845

4063
5014
1094
1512
3908
2829
1684
2446
2525
4476
1531
2960
1639
5199
2512

6268
4331
3263
5532
7380
4504
7857
2659
7366
3552
6191

3567
6932
6243
6254
3787
1181
3960
2629
1908
4093
2494
6776
6950

23529
31252

59972

5539
1365
4993
2639
3728
5255
1987
6113
5795
5549
2504
2657
4474
5051
4145

4096
3034
7454
5203
7155
4541
6407
1518
5794
1689
5509";
