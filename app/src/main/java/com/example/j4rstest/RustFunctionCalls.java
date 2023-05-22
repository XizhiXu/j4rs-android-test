package com.example.j4rstest;

import org.astonbitecode.j4rs.api.Instance;
import org.astonbitecode.j4rs.api.java2rust.Java2RustUtils;

public class RustFunctionCalls {

    private static native Instance<Integer> addintegers(Instance<Integer> i1, Instance<Integer> i2);
    private static native Instance<Long> addlongs(Instance<Long> i1, Instance<Long> i2);
    private static native Instance<String> fnstring(Instance<String> s);

    // region dto1
    private static native Instance<com.example.j4rstest.api.dto.TestDto0> fncustomobject10(Instance<com.example.j4rstest.api.dto.TestDto0> dto);
    private static native Instance<com.example.j4rstest.api.dto.TestDto1> fncustomobject11(Instance<com.example.j4rstest.api.dto.TestDto1> dto);
    private static native Instance<com.example.j4rstest.api.dto.TestDto2> fncustomobject12(Instance<com.example.j4rstest.api.dto.TestDto2> dto);
    private static native Instance<com.example.j4rstest.api.dto.TestDto3> fncustomobject13(Instance<com.example.j4rstest.api.dto.TestDto3> dto);
    private static native Instance<com.example.j4rstest.api.dto.TestDto4> fncustomobject14(Instance<com.example.j4rstest.api.dto.TestDto4> dto);
    private static native Instance<com.example.j4rstest.api.dto.TestDto5> fncustomobject15(Instance<com.example.j4rstest.api.dto.TestDto5> dto);
    private static native Instance<com.example.j4rstest.api.dto.TestDto6> fncustomobject16(Instance<com.example.j4rstest.api.dto.TestDto6> dto);
    private static native Instance<com.example.j4rstest.api.dto.TestDto7> fncustomobject17(Instance<com.example.j4rstest.api.dto.TestDto7> dto);
    private static native Instance<com.example.j4rstest.api.dto.TestDto8> fncustomobject18(Instance<com.example.j4rstest.api.dto.TestDto8> dto);
    private static native Instance<com.example.j4rstest.api.dto.TestDto9> fncustomobject19(Instance<com.example.j4rstest.api.dto.TestDto9> dto);
    // endregion

    // region dto2
    private static native Instance<com.example.j4rstest.api.dto2.TestDto0> fncustomobject20(Instance<com.example.j4rstest.api.dto2.TestDto0> dto);
    private static native Instance<com.example.j4rstest.api.dto2.TestDto1> fncustomobject21(Instance<com.example.j4rstest.api.dto2.TestDto1> dto);
    private static native Instance<com.example.j4rstest.api.dto2.TestDto2> fncustomobject22(Instance<com.example.j4rstest.api.dto2.TestDto2> dto);
    private static native Instance<com.example.j4rstest.api.dto2.TestDto3> fncustomobject23(Instance<com.example.j4rstest.api.dto2.TestDto3> dto);
    private static native Instance<com.example.j4rstest.api.dto2.TestDto4> fncustomobject24(Instance<com.example.j4rstest.api.dto2.TestDto4> dto);
    private static native Instance<com.example.j4rstest.api.dto2.TestDto5> fncustomobject25(Instance<com.example.j4rstest.api.dto2.TestDto5> dto);
    private static native Instance<com.example.j4rstest.api.dto2.TestDto6> fncustomobject26(Instance<com.example.j4rstest.api.dto2.TestDto6> dto);
    private static native Instance<com.example.j4rstest.api.dto2.TestDto7> fncustomobject27(Instance<com.example.j4rstest.api.dto2.TestDto7> dto);
    private static native Instance<com.example.j4rstest.api.dto2.TestDto8> fncustomobject28(Instance<com.example.j4rstest.api.dto2.TestDto8> dto);
    private static native Instance<com.example.j4rstest.api.dto2.TestDto9> fncustomobject29(Instance<com.example.j4rstest.api.dto2.TestDto9> dto);
    // endregion

    // region dto3
    private static native Instance<com.example.j4rstest.api.dto3.TestDto0> fncustomobject30(Instance<com.example.j4rstest.api.dto3.TestDto0> dto);
    private static native Instance<com.example.j4rstest.api.dto3.TestDto1> fncustomobject31(Instance<com.example.j4rstest.api.dto3.TestDto1> dto);
    private static native Instance<com.example.j4rstest.api.dto3.TestDto2> fncustomobject32(Instance<com.example.j4rstest.api.dto3.TestDto2> dto);
    private static native Instance<com.example.j4rstest.api.dto3.TestDto3> fncustomobject33(Instance<com.example.j4rstest.api.dto3.TestDto3> dto);
    private static native Instance<com.example.j4rstest.api.dto3.TestDto4> fncustomobject34(Instance<com.example.j4rstest.api.dto3.TestDto4> dto);
    private static native Instance<com.example.j4rstest.api.dto3.TestDto5> fncustomobject35(Instance<com.example.j4rstest.api.dto3.TestDto5> dto);
    private static native Instance<com.example.j4rstest.api.dto3.TestDto6> fncustomobject36(Instance<com.example.j4rstest.api.dto3.TestDto6> dto);
    private static native Instance<com.example.j4rstest.api.dto3.TestDto7> fncustomobject37(Instance<com.example.j4rstest.api.dto3.TestDto7> dto);
    private static native Instance<com.example.j4rstest.api.dto3.TestDto8> fncustomobject38(Instance<com.example.j4rstest.api.dto3.TestDto8> dto);
    private static native Instance<com.example.j4rstest.api.dto3.TestDto9> fncustomobject39(Instance<com.example.j4rstest.api.dto3.TestDto9> dto);
    // endregion

    // region dto4
    private static native Instance<com.example.j4rstest.api.dto4.TestDto0> fncustomobject40(Instance<com.example.j4rstest.api.dto4.TestDto0> dto);
    private static native Instance<com.example.j4rstest.api.dto4.TestDto1> fncustomobject41(Instance<com.example.j4rstest.api.dto4.TestDto1> dto);
    private static native Instance<com.example.j4rstest.api.dto4.TestDto2> fncustomobject42(Instance<com.example.j4rstest.api.dto4.TestDto2> dto);
    private static native Instance<com.example.j4rstest.api.dto4.TestDto3> fncustomobject43(Instance<com.example.j4rstest.api.dto4.TestDto3> dto);
    private static native Instance<com.example.j4rstest.api.dto4.TestDto4> fncustomobject44(Instance<com.example.j4rstest.api.dto4.TestDto4> dto);
    private static native Instance<com.example.j4rstest.api.dto4.TestDto5> fncustomobject45(Instance<com.example.j4rstest.api.dto4.TestDto5> dto);
    private static native Instance<com.example.j4rstest.api.dto4.TestDto6> fncustomobject46(Instance<com.example.j4rstest.api.dto4.TestDto6> dto);
    private static native Instance<com.example.j4rstest.api.dto4.TestDto7> fncustomobject47(Instance<com.example.j4rstest.api.dto4.TestDto7> dto);
    private static native Instance<com.example.j4rstest.api.dto4.TestDto8> fncustomobject48(Instance<com.example.j4rstest.api.dto4.TestDto8> dto);
    private static native Instance<com.example.j4rstest.api.dto4.TestDto9> fncustomobject49(Instance<com.example.j4rstest.api.dto4.TestDto9> dto);
    // endregion

    // region dto5
    private static native Instance<com.example.j4rstest.api.dto5.TestDto0> fncustomobject50(Instance<com.example.j4rstest.api.dto5.TestDto0> dto);
    private static native Instance<com.example.j4rstest.api.dto5.TestDto1> fncustomobject51(Instance<com.example.j4rstest.api.dto5.TestDto1> dto);
    private static native Instance<com.example.j4rstest.api.dto5.TestDto2> fncustomobject52(Instance<com.example.j4rstest.api.dto5.TestDto2> dto);
    private static native Instance<com.example.j4rstest.api.dto5.TestDto3> fncustomobject53(Instance<com.example.j4rstest.api.dto5.TestDto3> dto);
    private static native Instance<com.example.j4rstest.api.dto5.TestDto4> fncustomobject54(Instance<com.example.j4rstest.api.dto5.TestDto4> dto);
    private static native Instance<com.example.j4rstest.api.dto5.TestDto5> fncustomobject55(Instance<com.example.j4rstest.api.dto5.TestDto5> dto);
    private static native Instance<com.example.j4rstest.api.dto5.TestDto6> fncustomobject56(Instance<com.example.j4rstest.api.dto5.TestDto6> dto);
    private static native Instance<com.example.j4rstest.api.dto5.TestDto7> fncustomobject57(Instance<com.example.j4rstest.api.dto5.TestDto7> dto);
    private static native Instance<com.example.j4rstest.api.dto5.TestDto8> fncustomobject58(Instance<com.example.j4rstest.api.dto5.TestDto8> dto);
    private static native Instance<com.example.j4rstest.api.dto5.TestDto9> fncustomobject59(Instance<com.example.j4rstest.api.dto5.TestDto9> dto);
    // endregion


    public RustFunctionCalls() {
        System.loadLibrary("j4rstest");
    }

    public Integer addInRust(Integer i1, Integer i2) {
        Instance<Integer> instance = addintegers(
                Java2RustUtils.createInstance(i1),
                Java2RustUtils.createInstance(i2));
        return Java2RustUtils.getObjectCasted(instance);
    }

    public Long addInRust(Long i1, Long i2) {
        Instance<Long> instance = addlongs(
                Java2RustUtils.createInstance(i1),
                Java2RustUtils.createInstance(i2));
        return Java2RustUtils.getObjectCasted(instance);
    }

    public String strings(String string) {
        Instance<String> i = fnstring(Java2RustUtils.createInstance(string));
        return Java2RustUtils.getObjectCasted(i);
    }

    // region function1
    public com.example.j4rstest.api.dto.TestDto0 doCallWithCustomObject1_0(com.example.j4rstest.api.dto.TestDto0 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject10(Java2RustUtils.createInstance(testDto)));
    }

    public com.example.j4rstest.api.dto.TestDto1 doCallWithCustomObject1_1(com.example.j4rstest.api.dto.TestDto1 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject11(Java2RustUtils.createInstance(testDto)));
    }

    public com.example.j4rstest.api.dto.TestDto2 doCallWithCustomObject1_2(com.example.j4rstest.api.dto.TestDto2 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject12(Java2RustUtils.createInstance(testDto)));
    }

    public com.example.j4rstest.api.dto.TestDto3 doCallWithCustomObject1_3(com.example.j4rstest.api.dto.TestDto3 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject13(Java2RustUtils.createInstance(testDto)));
    }

    public com.example.j4rstest.api.dto.TestDto4 doCallWithCustomObject1_4(com.example.j4rstest.api.dto.TestDto4 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject14(Java2RustUtils.createInstance(testDto)));
    }

    public com.example.j4rstest.api.dto.TestDto5 doCallWithCustomObject1_5(com.example.j4rstest.api.dto.TestDto5 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject15(Java2RustUtils.createInstance(testDto)));
    }

    public com.example.j4rstest.api.dto.TestDto6 doCallWithCustomObject1_6(com.example.j4rstest.api.dto.TestDto6 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject16(Java2RustUtils.createInstance(testDto)));
    }

    public com.example.j4rstest.api.dto.TestDto7 doCallWithCustomObject1_7(com.example.j4rstest.api.dto.TestDto7 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject17(Java2RustUtils.createInstance(testDto)));
    }

    public com.example.j4rstest.api.dto.TestDto8 doCallWithCustomObject1_8(com.example.j4rstest.api.dto.TestDto8 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject18(Java2RustUtils.createInstance(testDto)));
    }

    public com.example.j4rstest.api.dto.TestDto9 doCallWithCustomObject1_9(com.example.j4rstest.api.dto.TestDto9 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject19(Java2RustUtils.createInstance(testDto)));
    }
    // endregion

    // region function2
    public com.example.j4rstest.api.dto2.TestDto0 doCallWithCustomObject2_0(com.example.j4rstest.api.dto2.TestDto0 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject20(Java2RustUtils.createInstance(testDto)));
    }

    public com.example.j4rstest.api.dto2.TestDto1 doCallWithCustomObject2_1(com.example.j4rstest.api.dto2.TestDto1 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject21(Java2RustUtils.createInstance(testDto)));
    }

    public com.example.j4rstest.api.dto2.TestDto2 doCallWithCustomObject2_2(com.example.j4rstest.api.dto2.TestDto2 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject22(Java2RustUtils.createInstance(testDto)));
    }

    public com.example.j4rstest.api.dto2.TestDto3 doCallWithCustomObject2_3(com.example.j4rstest.api.dto2.TestDto3 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject23(Java2RustUtils.createInstance(testDto)));
    }

    public com.example.j4rstest.api.dto2.TestDto4 doCallWithCustomObject2_4(com.example.j4rstest.api.dto2.TestDto4 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject24(Java2RustUtils.createInstance(testDto)));
    }

    public com.example.j4rstest.api.dto2.TestDto5 doCallWithCustomObject2_5(com.example.j4rstest.api.dto2.TestDto5 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject25(Java2RustUtils.createInstance(testDto)));
    }

    public com.example.j4rstest.api.dto2.TestDto6 doCallWithCustomObject2_6(com.example.j4rstest.api.dto2.TestDto6 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject26(Java2RustUtils.createInstance(testDto)));
    }

    public com.example.j4rstest.api.dto2.TestDto7 doCallWithCustomObject2_7(com.example.j4rstest.api.dto2.TestDto7 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject27(Java2RustUtils.createInstance(testDto)));
    }

    public com.example.j4rstest.api.dto2.TestDto8 doCallWithCustomObject2_8(com.example.j4rstest.api.dto2.TestDto8 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject28(Java2RustUtils.createInstance(testDto)));
    }

    public com.example.j4rstest.api.dto2.TestDto9 doCallWithCustomObject2_9(com.example.j4rstest.api.dto2.TestDto9 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject29(Java2RustUtils.createInstance(testDto)));
    }
    // endregion

    // region function3
    public com.example.j4rstest.api.dto3.TestDto0 doCallWithCustomObject3_0(com.example.j4rstest.api.dto3.TestDto0 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject30(Java2RustUtils.createInstance(testDto)));
    }

    public com.example.j4rstest.api.dto3.TestDto1 doCallWithCustomObject3_1(com.example.j4rstest.api.dto3.TestDto1 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject31(Java2RustUtils.createInstance(testDto)));
    }

    public com.example.j4rstest.api.dto3.TestDto2 doCallWithCustomObject3_2(com.example.j4rstest.api.dto3.TestDto2 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject32(Java2RustUtils.createInstance(testDto)));
    }

    public com.example.j4rstest.api.dto3.TestDto3 doCallWithCustomObject3_3(com.example.j4rstest.api.dto3.TestDto3 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject33(Java2RustUtils.createInstance(testDto)));
    }

    public com.example.j4rstest.api.dto3.TestDto4 doCallWithCustomObject3_4(com.example.j4rstest.api.dto3.TestDto4 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject34(Java2RustUtils.createInstance(testDto)));
    }

    public com.example.j4rstest.api.dto3.TestDto5 doCallWithCustomObject3_5(com.example.j4rstest.api.dto3.TestDto5 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject35(Java2RustUtils.createInstance(testDto)));
    }

    public com.example.j4rstest.api.dto3.TestDto6 doCallWithCustomObject3_6(com.example.j4rstest.api.dto3.TestDto6 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject36(Java2RustUtils.createInstance(testDto)));
    }

    public com.example.j4rstest.api.dto3.TestDto7 doCallWithCustomObject3_7(com.example.j4rstest.api.dto3.TestDto7 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject37(Java2RustUtils.createInstance(testDto)));
    }

    public com.example.j4rstest.api.dto3.TestDto8 doCallWithCustomObject3_8(com.example.j4rstest.api.dto3.TestDto8 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject38(Java2RustUtils.createInstance(testDto)));
    }

    public com.example.j4rstest.api.dto3.TestDto9 doCallWithCustomObject3_9(com.example.j4rstest.api.dto3.TestDto9 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject39(Java2RustUtils.createInstance(testDto)));
    }
    // endregion

    // region function4
    public com.example.j4rstest.api.dto4.TestDto0 doCallWithCustomObject4_0(com.example.j4rstest.api.dto4.TestDto0 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject40(Java2RustUtils.createInstance(testDto)));
    }

    public com.example.j4rstest.api.dto4.TestDto1 doCallWithCustomObject4_1(com.example.j4rstest.api.dto4.TestDto1 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject41(Java2RustUtils.createInstance(testDto)));
    }

    public com.example.j4rstest.api.dto4.TestDto2 doCallWithCustomObject4_2(com.example.j4rstest.api.dto4.TestDto2 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject42(Java2RustUtils.createInstance(testDto)));
    }

    public com.example.j4rstest.api.dto4.TestDto3 doCallWithCustomObject4_3(com.example.j4rstest.api.dto4.TestDto3 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject43(Java2RustUtils.createInstance(testDto)));
    }

    public com.example.j4rstest.api.dto4.TestDto4 doCallWithCustomObject4_4(com.example.j4rstest.api.dto4.TestDto4 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject44(Java2RustUtils.createInstance(testDto)));
    }

    public com.example.j4rstest.api.dto4.TestDto5 doCallWithCustomObject4_5(com.example.j4rstest.api.dto4.TestDto5 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject45(Java2RustUtils.createInstance(testDto)));
    }

    public com.example.j4rstest.api.dto4.TestDto6 doCallWithCustomObject4_6(com.example.j4rstest.api.dto4.TestDto6 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject46(Java2RustUtils.createInstance(testDto)));
    }

    public com.example.j4rstest.api.dto4.TestDto7 doCallWithCustomObject4_7(com.example.j4rstest.api.dto4.TestDto7 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject47(Java2RustUtils.createInstance(testDto)));
    }

    public com.example.j4rstest.api.dto4.TestDto8 doCallWithCustomObject4_8(com.example.j4rstest.api.dto4.TestDto8 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject48(Java2RustUtils.createInstance(testDto)));
    }

    public com.example.j4rstest.api.dto4.TestDto9 doCallWithCustomObject4_9(com.example.j4rstest.api.dto4.TestDto9 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject49(Java2RustUtils.createInstance(testDto)));
    }
    // endregion

    // region function5
    public com.example.j4rstest.api.dto5.TestDto0 doCallWithCustomObject5_0(com.example.j4rstest.api.dto5.TestDto0 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject50(Java2RustUtils.createInstance(testDto)));
    }

    public com.example.j4rstest.api.dto5.TestDto1 doCallWithCustomObject5_1(com.example.j4rstest.api.dto5.TestDto1 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject51(Java2RustUtils.createInstance(testDto)));
    }

    public com.example.j4rstest.api.dto5.TestDto2 doCallWithCustomObject5_2(com.example.j4rstest.api.dto5.TestDto2 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject52(Java2RustUtils.createInstance(testDto)));
    }

    public com.example.j4rstest.api.dto5.TestDto3 doCallWithCustomObject5_3(com.example.j4rstest.api.dto5.TestDto3 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject53(Java2RustUtils.createInstance(testDto)));
    }

    public com.example.j4rstest.api.dto5.TestDto4 doCallWithCustomObject5_4(com.example.j4rstest.api.dto5.TestDto4 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject54(Java2RustUtils.createInstance(testDto)));
    }

    public com.example.j4rstest.api.dto5.TestDto5 doCallWithCustomObject5_5(com.example.j4rstest.api.dto5.TestDto5 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject55(Java2RustUtils.createInstance(testDto)));
    }

    public com.example.j4rstest.api.dto5.TestDto6 doCallWithCustomObject5_6(com.example.j4rstest.api.dto5.TestDto6 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject56(Java2RustUtils.createInstance(testDto)));
    }

    public com.example.j4rstest.api.dto5.TestDto7 doCallWithCustomObject5_7(com.example.j4rstest.api.dto5.TestDto7 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject57(Java2RustUtils.createInstance(testDto)));
    }

    public com.example.j4rstest.api.dto5.TestDto8 doCallWithCustomObject5_8(com.example.j4rstest.api.dto5.TestDto8 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject58(Java2RustUtils.createInstance(testDto)));
    }

    public com.example.j4rstest.api.dto5.TestDto9 doCallWithCustomObject5_9(com.example.j4rstest.api.dto5.TestDto9 testDto) {
        return Java2RustUtils.getObjectCasted(fncustomobject59(Java2RustUtils.createInstance(testDto)));
    }
    // endregion
}
