/** Ajv 8.20 standalone code generation for the non-mutating contract profile.
 * The properties/required emitters follow Ajv's MIT-licensed keyword emitters
 * (https://github.com/ajv-validator/ajv), using independent validity temporaries
 * and explicit missing-property branches instead of assignments in conditions.
 * Copyright (c) 2015-2021 Evgeny Poberezkin; see AJV-LICENSE.
 * Keep the differential tests when upgrading Ajv: this uses its codegen API.
 */
import code from 'ajv/dist/vocabularies/code.js';
import util from 'ajv/dist/compile/util.js';
import {_} from 'ajv/dist/compile/codegen/index.js';
import ts from 'typescript';
export function configureContractCodegen(ajv) {
  if (ajv.opts.allErrors || ajv.opts.$data || ajv.opts.useDefaults ||
      ajv.opts.removeAdditional) {
    throw new Error('Contract code generation requires the reviewed non-mutating, fail-fast profile');
  }
  // Definitions are instance-owned copies. Retain their rule order so
  // fail-fast error selection stays identical to unmodified Ajv.
  const properties=ajv.getKeyword('properties');
  properties.code = (cxt)=>{
    const {gen,schema,data,it}=cxt;
    const all=code.allSchemaProperties(schema);
    for(const prop of all)it.definedProperties.add(prop);
    if(it.opts.unevaluated && all.length && it.props!==true)
      it.props=util.mergeEvaluated.props(gen,util.toHash(all),it.props);
    for(const prop of all.filter(p=>!util.alwaysValidSchema(it,schema[p]))) {
      const valid=gen.name('valid');
      gen.if(code.propertyInData(gen,data,prop,it.opts.ownProperties));
      cxt.subschema({keyword:'properties',schemaProp:prop,dataProp:prop},valid);
      if(!it.allErrors)gen.else().var(valid,true);
      gen.endIf();cxt.it.definedProperties.add(prop);cxt.ok(valid);
    }
  };
  const required=ajv.getKeyword('required');
  required.code = (cxt)=>{
    if(cxt.it.opts.strictRequired) {
      for(const prop of cxt.schema) {
        if(cxt.parentSchema.properties?.[prop]===undefined && !cxt.it.definedProperties.has(prop)) {
          const schemaPath=cxt.it.schemaEnv.baseId+cxt.it.errSchemaPath;
          util.checkStrictMode(cxt.it,`required property "${prop}" is not defined at "${schemaPath}" (strictRequired)`,cxt.it.opts.strictRequired);
        }
      }
    }
    for(const prop of cxt.schema) {
      cxt.gen.if(code.noPropertyInData(cxt.gen,cxt.data,prop,cxt.it.opts.ownProperties));
      code.reportMissingProp(cxt,_`${prop}`);
      cxt.gen.else();
    }
  };
}
/** Internal error accumulators always use arrays. Public wrappers still expose
 * null on success. Failed nested validators supply error arrays; union rollback
 * truncates the accumulator to its previous length, including zero. This removes
 * nullable allocation guards without discarding or exposing error contents.
 */
export function normalizeErrorAccumulators(source) {
  const ast=ts.createSourceFile('validators.js',source,ts.ScriptTarget.ES2020,true,ts.ScriptKind.JS);
  const isErrors=n=>ts.isIdentifier(n)&&n.text==='vErrors';
  const isNull=n=>n?.kind===ts.SyntaxKind.NullKeyword;
  const result=ts.transform(ast,[ctx=>root=>{
    const visit=node=>{
      if(ts.isIfStatement(node)&&ts.isIdentifier(node.expression)&&node.elseStatement) {
        const unwrap=n=>ts.isBlock(n)&&n.statements.length===1?n.statements[0]:n;
        const yes=unwrap(node.thenStatement),no=unwrap(node.elseStatement);
        if(ts.isExpressionStatement(yes)&&ts.isBinaryExpression(yes.expression)&&ts.isExpressionStatement(no)&&ts.isBinaryExpression(no.expression)) {
          const a=yes.expression,b=no.expression;
          if(a.operatorToken.kind===ts.SyntaxKind.EqualsToken&&ts.isPropertyAccessExpression(a.left)&&isErrors(a.left.expression)&&a.left.name.text==='length'&&ts.isIdentifier(a.right)&&a.right.text===node.expression.text&&b.operatorToken.kind===ts.SyntaxKind.EqualsToken&&isErrors(b.left)&&isNull(b.right))return yes;
        }
      }
      if(ts.isVariableDeclaration(node)&&isErrors(node.name)&&isNull(node.initializer))
        return ctx.factory.updateVariableDeclaration(node,node.name,node.exclamationToken,node.type,ctx.factory.createArrayLiteralExpression());
      if(ts.isBinaryExpression(node)&&isErrors(node.left)&&isNull(node.right)) {
        if(node.operatorToken.kind===ts.SyntaxKind.EqualsToken)
          return ctx.factory.updateBinaryExpression(node,node.left,node.operatorToken,ctx.factory.createArrayLiteralExpression());
        if(node.operatorToken.kind===ts.SyntaxKind.EqualsEqualsEqualsToken)return ctx.factory.createFalse();
        if(node.operatorToken.kind===ts.SyntaxKind.ExclamationEqualsEqualsToken)return ctx.factory.createTrue();
      }
      return ts.visitEachChild(node,visit,ctx);
    };return ts.visitNode(root,visit);
  }]);
  try{return ts.createPrinter().printFile(result.transformed[0]);}finally{result.dispose();}
}
