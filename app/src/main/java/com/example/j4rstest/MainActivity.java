package com.example.j4rstest;

import android.os.Bundle;
import android.util.Log;
import android.view.View;
import android.widget.TextView;

import androidx.appcompat.app.AppCompatActivity;

public class MainActivity extends AppCompatActivity {
    private static String TAG = "J4RS";

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);
        View v = getWindow().getDecorView();
        TextView title = v.findViewById(R.id.mytextview);
        RustFunctionCalls rfc = new RustFunctionCalls();
        String s = rfc.strings("Hi");
        Log.d(TAG, "--------------" + s);

        Integer i = rfc.addInRust(111, 222);
        Log.d(TAG, "--------------" + i);

        Long l = rfc.addInRust(11L, 22L);
        Log.d(TAG, "--------------" + l);
        
        groupCallDto1(rfc, title);
//        groupCallDto2(rfc, title);
//        groupCallDto3(rfc, title);
//        groupCallDto4(rfc, title);
//        groupCallDto5(rfc, title);
    }

    // region dto1
    private void groupCallDto1(RustFunctionCalls rfc, TextView title) {
        callDto1_0(rfc, title);
        callDto1_1(rfc, title);
        callDto1_2(rfc, title);
        callDto1_3(rfc, title);
        callDto1_4(rfc, title);
        callDto1_5(rfc, title);
        callDto1_6(rfc, title);
        callDto1_7(rfc, title);
        callDto1_8(rfc, title);
        callDto1_9(rfc, title);
    }
    private void callDto1_0(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject1_0(new com.example.j4rstest.api.dto.TestDto0(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    private void callDto1_1(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject1_1(new com.example.j4rstest.api.dto.TestDto1(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    private void callDto1_2(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject1_2(new com.example.j4rstest.api.dto.TestDto2(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    private void callDto1_3(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject1_3(new com.example.j4rstest.api.dto.TestDto3(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    private void callDto1_4(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject1_4(new com.example.j4rstest.api.dto.TestDto4(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    private void callDto1_5(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject1_5(new com.example.j4rstest.api.dto.TestDto5(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    private void callDto1_6(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject1_6(new com.example.j4rstest.api.dto.TestDto6(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    private void callDto1_7(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject1_7(new com.example.j4rstest.api.dto.TestDto7(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    private void callDto1_8(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject1_8(new com.example.j4rstest.api.dto.TestDto8(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    private void callDto1_9(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject1_9(new com.example.j4rstest.api.dto.TestDto9(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    // endregion

    // region dto2
    private void groupCallDto2(RustFunctionCalls rfc, TextView title) {
        callDto2_0(rfc, title);
        callDto2_1(rfc, title);
        callDto2_2(rfc, title);
        callDto2_3(rfc, title);
        callDto2_4(rfc, title);
        callDto2_5(rfc, title);
        callDto2_6(rfc, title);
        callDto2_7(rfc, title);
        callDto2_8(rfc, title);
        callDto2_9(rfc, title);
    }
    private void callDto2_0(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject2_0(new com.example.j4rstest.api.dto2.TestDto0(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    private void callDto2_1(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject2_1(new com.example.j4rstest.api.dto2.TestDto1(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    private void callDto2_2(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject2_2(new com.example.j4rstest.api.dto2.TestDto2(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    private void callDto2_3(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject2_3(new com.example.j4rstest.api.dto2.TestDto3(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    private void callDto2_4(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject2_4(new com.example.j4rstest.api.dto2.TestDto4(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    private void callDto2_5(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject2_5(new com.example.j4rstest.api.dto2.TestDto5(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    private void callDto2_6(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject2_6(new com.example.j4rstest.api.dto2.TestDto6(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    private void callDto2_7(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject2_7(new com.example.j4rstest.api.dto2.TestDto7(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    private void callDto2_8(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject2_8(new com.example.j4rstest.api.dto2.TestDto8(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    private void callDto2_9(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject2_9(new com.example.j4rstest.api.dto2.TestDto9(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    // endregion

    // region dto3
    private void groupCallDto3(RustFunctionCalls rfc, TextView title) {
        callDto3_0(rfc, title);
        callDto3_1(rfc, title);
        callDto3_2(rfc, title);
        callDto3_3(rfc, title);
        callDto3_4(rfc, title);
        callDto3_5(rfc, title);
        callDto3_6(rfc, title);
        callDto3_7(rfc, title);
        callDto3_8(rfc, title);
        callDto3_9(rfc, title);
    }
    private void callDto3_0(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject3_0(new com.example.j4rstest.api.dto3.TestDto0(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    private void callDto3_1(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject3_1(new com.example.j4rstest.api.dto3.TestDto1(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    private void callDto3_2(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject3_2(new com.example.j4rstest.api.dto3.TestDto2(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    private void callDto3_3(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject3_3(new com.example.j4rstest.api.dto3.TestDto3(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    private void callDto3_4(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject3_4(new com.example.j4rstest.api.dto3.TestDto4(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    private void callDto3_5(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject3_5(new com.example.j4rstest.api.dto3.TestDto5(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    private void callDto3_6(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject3_6(new com.example.j4rstest.api.dto3.TestDto6(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    private void callDto3_7(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject3_7(new com.example.j4rstest.api.dto3.TestDto7(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    private void callDto3_8(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject3_8(new com.example.j4rstest.api.dto3.TestDto8(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    private void callDto3_9(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject3_9(new com.example.j4rstest.api.dto3.TestDto9(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    // endregion

    // region dto4
    private void groupCallDto4(RustFunctionCalls rfc, TextView title) {
        callDto4_0(rfc, title);
        callDto4_1(rfc, title);
        callDto4_2(rfc, title);
        callDto4_3(rfc, title);
        callDto4_4(rfc, title);
        callDto4_5(rfc, title);
        callDto4_6(rfc, title);
        callDto4_7(rfc, title);
        callDto4_8(rfc, title);
        callDto4_9(rfc, title);
    }
    private void callDto4_0(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject4_0(new com.example.j4rstest.api.dto4.TestDto0(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    private void callDto4_1(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject4_1(new com.example.j4rstest.api.dto4.TestDto1(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    private void callDto4_2(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject4_2(new com.example.j4rstest.api.dto4.TestDto2(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    private void callDto4_3(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject4_3(new com.example.j4rstest.api.dto4.TestDto3(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    private void callDto4_4(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject4_4(new com.example.j4rstest.api.dto4.TestDto4(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    private void callDto4_5(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject4_5(new com.example.j4rstest.api.dto4.TestDto5(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    private void callDto4_6(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject4_6(new com.example.j4rstest.api.dto4.TestDto6(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    private void callDto4_7(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject4_7(new com.example.j4rstest.api.dto4.TestDto7(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    private void callDto4_8(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject4_8(new com.example.j4rstest.api.dto4.TestDto8(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    private void callDto4_9(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject4_9(new com.example.j4rstest.api.dto4.TestDto9(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    // endregion

    // region dto5
    private void groupCallDto5(RustFunctionCalls rfc, TextView title) {
        callDto5_0(rfc, title);
        callDto5_1(rfc, title);
        callDto5_2(rfc, title);
        callDto5_3(rfc, title);
        callDto5_4(rfc, title);
        callDto5_5(rfc, title);
        callDto5_6(rfc, title);
        callDto5_7(rfc, title);
        callDto5_8(rfc, title);
        callDto5_9(rfc, title);
    }
    private void callDto5_0(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject5_0(new com.example.j4rstest.api.dto5.TestDto0(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    private void callDto5_1(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject5_1(new com.example.j4rstest.api.dto5.TestDto1(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    private void callDto5_2(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject5_2(new com.example.j4rstest.api.dto5.TestDto2(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    private void callDto5_3(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject5_3(new com.example.j4rstest.api.dto5.TestDto3(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    private void callDto5_4(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject5_4(new com.example.j4rstest.api.dto5.TestDto4(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    private void callDto5_5(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject5_5(new com.example.j4rstest.api.dto5.TestDto5(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    private void callDto5_6(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject5_6(new com.example.j4rstest.api.dto5.TestDto6(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    private void callDto5_7(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject5_7(new com.example.j4rstest.api.dto5.TestDto7(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    private void callDto5_8(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject5_8(new com.example.j4rstest.api.dto5.TestDto8(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    private void callDto5_9(RustFunctionCalls rfc, TextView title) {
        int number = rfc.doCallWithCustomObject5_9(new com.example.j4rstest.api.dto5.TestDto9(33)).getNumber();
        title.setText("The result is " + number);
        Log.d(TAG, "--------------" + number);
    }
    // endregion
}